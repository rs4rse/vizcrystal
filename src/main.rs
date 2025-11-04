use bevy::prelude::*;

use vizmat::io::load_crystal;
use vizmat::ui::{camera_controls, setup_camera, setup_scene};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Entry point for WASM
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    run_app();
}

/// Entry point for desktop
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run_app();
}

/// Shared function for Bevy app setup
fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_crystal, setup_scene, setup_camera).chain())
        .add_systems(Update, camera_controls)
        .run();
}
