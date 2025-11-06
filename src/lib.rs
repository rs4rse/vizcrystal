use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub(crate) mod io;
pub(crate) mod ui;

pub(crate) mod constants;
pub(crate) mod parse;
pub(crate) mod structure;

use crate::io::{
    handle_file_drag_drop, load_default_crystal, load_dropped_file, update_crystal_from_file,
    FileDragDrop,
};
use crate::ui::{
    camera_controls, setup_camera, setup_file_ui, setup_scene, update_file_ui, update_scene,
};

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
        .init_resource::<FileDragDrop>()
        .add_event::<bevy::window::FileDragAndDrop>()
        .add_systems(
            Startup,
            (
                load_default_crystal,
                setup_scene,
                setup_camera,
                setup_file_ui,
            ),
        )
        .add_systems(
            Update,
            (
                handle_file_drag_drop,
                load_dropped_file,
                update_crystal_from_file,
                update_file_ui,
                update_scene,
                camera_controls,
            ),
        )
        .run();
}
