use bevy::prelude::*;
// Structure to represent an atom from XYZ file
// `#` is a macro. no inheritance. close to python decorator. injecting on top of something.
// traits are like interfaces.
#[derive(Debug)]
pub struct Atom {
    pub element: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Structure to hold our crystal data
#[derive(Resource)]
pub struct Crystal {
    pub atoms: Vec<Atom>,
}

// Component to mark atom entities
#[derive(Component)]
pub struct AtomEntity;
