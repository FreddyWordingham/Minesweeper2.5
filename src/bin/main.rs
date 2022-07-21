use bevy::{log, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::{
    resources::{BoardAssets, BoardOptions, SpriteMaterial},
    BoardPlugin,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bin/main.js")]
extern "C" {
    fn alert_game_over() -> String;
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "SimRPG!".to_string(),
        width: 1200.,
        height: 800.,
        ..default()
    })
    .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(setup_board)
        .add_state(AppState::Out)
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(state_handler)
        .add_system(completion_checker);
    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::R) {
        log::debug!("reset detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 100,
        tile_padding: 1.0,
        safe_start: true,
        ..default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            colour: Color::WHITE,
            ..default()
        },
        tile_material: SpriteMaterial {
            colour: Color::DARK_GRAY,
            ..default()
        },
        covered_tile_material: SpriteMaterial {
            colour: Color::GRAY,
            ..default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colours: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            colour: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            colour: Color::WHITE,
        },
    });
    // Plugin activation
    state.set(AppState::InGame).unwrap();
}

use board_plugin::events::BombExplosionEvent;
fn completion_checker(mut bomb_explosion_event_reader: EventReader<BombExplosionEvent>) {
    for event in bomb_explosion_event_reader.iter() {
        log::info!("Bomb explosion event: {:?}", event);
        alert_game_over();
    }
}
