use bevy::{log, prelude::*};

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
    app.insert_resource(BoardOptions {
        safe_start: true,
        map_size: (50, 25),
        bomb_count: 100,
        tile_padding: 1.0,
        ..default()
    })
    .add_state(AppState::InGame)
    .add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
    .add_system(bevy::input::system::exit_on_esc_system)
    .add_system(state_handler);
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
