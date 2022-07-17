use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::{resources::BoardOptions, BoardPlugin};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "SimRPG!".to_string(),
        width: 600.,
        height: 400.,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .insert_resource(BoardOptions {
        map_size: (50, 40),
        bomb_count: 400,
        tile_padding: 1.0,
        ..default()
    })
    .add_plugin(BoardPlugin)
    .add_system(bevy::input::system::exit_on_esc_system);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
