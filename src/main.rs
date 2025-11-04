use bevy::prelude::*;

use vizmat::io::load_crystal;
use vizmat::ui::{camera_controls, setup_camera, setup_scene};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_crystal, setup_scene, setup_camera).chain())
        .add_systems(Update, camera_controls)
        .run();
}
