use bevy::ecs::system::Commands;

use crate::structure::{Atom, Crystal};

// System to load crystal data
pub fn load_crystal(mut commands: Commands) {
    // For now, use the default water molecule structure
    // In the future, this can be extended to load from embedded assets or user input
    println!("Loading default water molecule structure");

    let crystal = Crystal {
        atoms: vec![
            Atom {
                element: "O".to_string(),
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Atom {
                element: "H".to_string(),
                x: 0.757,
                y: 0.587,
                z: 0.0,
            },
            Atom {
                element: "H".to_string(),
                x: -0.757,
                y: 0.587,
                z: 0.0,
            },
        ],
    };

    commands.insert_resource(crystal);
}
