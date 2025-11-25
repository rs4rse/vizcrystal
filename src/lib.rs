use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub(crate) mod io;
pub(crate) mod ui;

pub(crate) mod client;
pub(crate) mod constants;
pub(crate) mod parse;
pub(crate) mod structure;

use crate::client::{poll_websocket_stream, setup_websocket_stream};
use crate::io::{handle_file_drag_drop, load_dropped_file, update_crystal_from_file, FileDragDrop};
use crate::structure::{update_crystal_system, UpdateStructure};
use crate::ui::{
    camera_controls, handle_load_default_button, setup_camera, setup_file_ui, setup_scene,
    update_file_ui, update_scene,
};
use crate::ui::{camera_controls, refresh_atoms_system, setup_cameras, setup_scene};
use crate::ui::{
    handle_toggle_events, reset_camera_button_interaction, toggle_button, ToggleEvent, ToggleStates,
};
use crate::ui::{setup_buttons, spawn_axis};

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
        .init_resource::<ToggleStates>()
        .init_resource::<FileDragDrop>()
        .add_event::<UpdateStructure>()
        .add_event::<ToggleEvent>()
        .add_event::<bevy::window::FileDragAndDrop>()
        .add_systems(Startup, load_default_crystal)
        .add_systems(
            Startup,
            (
                setup_cameras,
                spawn_axis,
                setup_buttons,
                setup_websocket_stream,
            )
                .after(load_default_crystal),
        )
        .add_systems(
            Update,
            (
                poll_websocket_stream,
                update_crystal_system,
                handle_file_drag_drop,
                load_dropped_file,
                update_crystal_from_file,
                update_file_ui,
                refresh_atoms_system,
                toggle_button,
                reset_camera_button_interaction,
                handle_toggle_events,
                handle_load_default_button,
                camera_controls,
                update_scene,
            ),
        )
        .run();
}
