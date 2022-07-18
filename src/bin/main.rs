use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::{resources::BoardOptions, BoardPlugin};

fn main() {
    let mut app = App::new();
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.insert_resource(WindowDescriptor {
        title: "SimRPG!".to_string(),
        width: 1200.,
        height: 800.,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .insert_resource(BoardOptions {
        map_size: (40, 20),
        bomb_count: 200,
        tile_padding: 1.0,
        ..default()
    })
    .add_plugin(BoardPlugin)
    .add_system(bevy::input::system::exit_on_esc_system);
    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
