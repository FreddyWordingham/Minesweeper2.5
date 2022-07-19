use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use board_plugin::{resources::BoardOptions, BoardPlugin};

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen(module = "/src/bin/main.js")]
// extern "C" {
//     fn name() -> String;
// }

// // lifted from the `console_log` example
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

fn main() {
    // log(&format!("Hello! {}", name()));

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
    app.insert_resource(BoardOptions {
        map_size: (400, 20),
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
