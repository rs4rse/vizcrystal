use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub(crate) mod io;
pub(crate) mod ui;

pub(crate) mod constants;
pub(crate) mod parse;
pub(crate) mod structure;

use crate::io::load_crystal;
use crate::ui::{camera_controls, setup_cameras, setup_scene, spawn_axis};

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
        .add_systems(Startup, load_crystal)
        .add_systems(Startup, setup_scene.after(load_crystal))
        .add_systems(Startup, (setup_cameras, spawn_axis).after(setup_scene))
        .add_systems(Update, camera_controls)
        .run();
}
