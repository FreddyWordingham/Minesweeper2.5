use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "SimRPG!".to_string(),
        width: 600.,
        height: 400.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
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
