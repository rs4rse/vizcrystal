use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub(crate) mod io;
pub(crate) mod ui;

pub(crate) mod constants;
pub(crate) mod parse;
pub(crate) mod structure;
pub(crate) mod client;

use crate::client::{poll_websocket_stream, setup_websocket_stream};
use crate::io::load_crystal;
use crate::structure::{update_crystal_system, UpdateStructure};
use crate::ui::{camera_controls, refresh_atoms_system, setup_camera, setup_scene};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Entry point for WASM
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start() {
    run_app();
}

/// Shared function for Bevy app setup
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
            custom_layer: |_| None,
        }))
        .add_event::<UpdateStructure>()
        .add_systems(
            Startup,
            (load_crystal, setup_scene, setup_camera, setup_websocket_stream).chain(),
        )
        .add_systems(
            Update,
            (
                poll_websocket_stream,
                update_crystal_system,
                refresh_atoms_system,
                camera_controls,
            ),
        )
        .run();
}
