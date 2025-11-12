use bevy::prelude::*;

// Structure to represent an atom from XYZ file
// `#` is a macro. no inheritance. close to python decorator. injecting on top of something.
// traits are like interfaces.
#[derive(Debug)]
pub(crate) struct Atom {
    pub element: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Resource)]
pub(crate) struct Molecule {
    pub atoms: Vec<Atom>,
}

// Component to mark atom entities
#[derive(Component)]
pub(crate) struct AtomEntity;

pub(crate) struct Lattice {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Lattice {
    pub(crate) fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Lattice { a, b, c }
    }
}

#[derive(Resource)]
pub(crate) struct Crystal {
    /// XXX: internal data structure can use crystal from ccmat-core. then with a convert to the ui.
    pub(crate) lattice: Lattice,
    pub(crate) atoms: Vec<Atom>,
}
