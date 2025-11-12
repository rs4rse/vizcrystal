use bevy::prelude::*;

use crate::structure::{Atom, Crystal, Lattice, Molecule};
// System to load structure data
pub fn load_structure(mut commands: Commands) {
    // For now, use the default water molecule structure
    // In the future, this can be extended to load from embedded assets or user input
    println!("Loading default water molecule structure");

    // let molecule = Molecule {
    //     atoms: vec![
    //         Atom {
    //             element: "O".to_string(),
    //             x: 0.0,
    //             y: 0.0,
    //             z: 0.0,
    //         },
    //         Atom {
    //             element: "H".to_string(),
    //             x: 0.757,
    //             y: 0.587,
    //             z: 0.0,
    //         },
    //         Atom {
    //             element: "H".to_string(),
    //             x: -0.757,
    //             y: 0.587,
    //             z: 0.0,
    //         },
    //     ],
    // };
    let a = Vec3::new(4., 0., 0.);
    let b = Vec3::new(0., 4., 0.);
    let c = Vec3::new(0., 0., 4.);

    let crystal = Crystal {
        lattice: Lattice::new(a, b, c),
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
