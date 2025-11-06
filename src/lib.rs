use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

pub(crate) mod io;
pub(crate) mod ui;

pub(crate) mod client;
pub(crate) mod constants;
pub(crate) mod parse;
pub(crate) mod structure;

use crate::client::{poll_websocket_stream, setup_websocket_stream};
use crate::io::load_crystal;
use crate::structure::{update_crystal_system, UpdateStructure};
use crate::ui::spawn_axis;
use crate::ui::{camera_controls, refresh_atoms_system, setup_cameras, setup_scene};
use crate::ui::{toggle_button_interaction, ToggleEvent, ToggleStates};

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
        .add_event::<UpdateStructure>()
        .add_event::<ToggleEvent>()
        .add_systems(Startup, load_crystal)
        .add_systems(Startup, setup_scene.after(load_crystal))
        .add_systems(
            Startup,
            (setup_cameras, spawn_axis, setup_websocket_stream).after(setup_scene),
        )
        .add_systems(
            Update,
            (
                poll_websocket_stream,
                update_crystal_system,
                refresh_atoms_system,
                toggle_button_interaction,
                handle_toggle_events,
                camera_controls,
            ),
        )
        .run();
}
