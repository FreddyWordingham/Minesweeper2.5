#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::as_conversions,
    clippy::blanket_clippy_restriction_lints,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::modulo_arithmetic,
    clippy::multiple_crate_versions,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::panic,
    clippy::print_stdout,
    clippy::pub_use,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::unreachable,
    clippy::unwrap_used
)]

pub mod bounds;
pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use bevy::{log, math::Vec3Swizzles, prelude::*, utils::HashMap};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use bounds::Bounds;
#[cfg(feature = "debug")]
use components::Uncover;
use components::{Bomb, BombNeighbour, Coordinates, Uncover};
use events::TileTriggerEvent;
use resources::{Board, BoardOptions, BoardPosition, Tile, TileMap, TileSize};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    #[inline]
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board)
            .add_system(systems::input::input_handling)
            .add_system(systems::uncover::trigger_event_handler)
            .add_system(systems::uncover::uncover_tiles)
            .add_event::<TileTriggerEvent>();
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
            app.register_inspectable::<BombNeighbour>();
            app.register_inspectable::<Bomb>();
            app.register_inspectable::<Uncover>();
        }
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Res<WindowDescriptor>,
        asset_server: Res<AssetServer>,
    ) {
        let font = asset_server.load("fonts/pixeled.ttf");
        let bomb_image = asset_server.load("sprites/bomb.png");

        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size);
        tile_map.set_bombs(options.bomb_count);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptative_tile_size(
                &window,
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // We define the board anchor position (bottom left)
        let board_mins = match options.position {
            BoardPosition::Centered(offset) => {
                Vec3::new(-board_size.x * 0.5, -board_size.y * 0.5, 0.0) + offset
            }
            BoardPosition::Offset(p) => p,
        };

        let mut safe_start = None;
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_mins))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x * 0.5, board_size.y * 0.5, 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));
            })
            .with_children(|parent| {
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    Color::DARK_GRAY,
                    &mut covered_tiles,
                    &bomb_image,
                    &font,
                    &mut safe_start,
                );
            });

        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board::new(
            tile_map,
            covered_tiles,
            Bounds {
                mins: board_mins.xy(),
                size: board_size,
            },
            tile_size,
        ));
    }

    /// Spawn the tiles.
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        colour: Color,
        covered_tile_colour: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        bomb_image: &Handle<Image>,
        font: &Handle<Font>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for y in 0..tile_map.height() {
            for x in 0..tile_map.width() {
                let mut cmd = parent.spawn();
                let coordinates = Coordinates::new(x as u16, y as u16);

                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: colour,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32).mul_add(tile_size, tile_size * 0.5),
                        (y as f32).mul_add(tile_size, tile_size * 0.5),
                        1.0,
                    ),
                    ..default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(coordinates);

                cmd.with_children(|parent| {
                    let entity = parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                color: covered_tile_colour,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);

                    if safe_start_entity.is_none() && tile_map.map()[(x, y)] == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile_map.map()[(x, y)] {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(count) => {
                        cmd.insert(BombNeighbour::new(count));
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(Self::bomb_count_text_bundle(
                                count,
                                font.clone(),
                                tile_size - tile_padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    /// Generates the bomb counter text 2D Bundle for a given value.
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::CYAN,
                3 => Color::GREEN,
                4 => Color::YELLOW,
                5 => Color::ORANGE,
                6 => Color::PURPLE,
                _ => Color::RED,
            },
        );

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        }
    }

    /// Computes a tile size that matches the window according to the tile map size.
    fn adaptative_tile_size(
        window: &Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (board_width, board_height): (usize, usize),
    ) -> f32 {
        let max_width = window.width / board_width as f32;
        let max_heigth = window.height / board_height as f32;
        max_width.min(max_heigth).clamp(min, max)
    }
}
