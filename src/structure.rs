use bevy::ecs::component::Component;
use bevy::ecs::event::{Event, EventReader};
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;

// Structure to represent an atom from XYZ file
// `#` is a macro. no inheritance. close to python decorator. injecting on top of something.
// traits are like interfaces.
#[derive(Debug, Clone)]
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

// Event to update the structure with new atom positions
#[derive(Event, Clone)]
pub struct UpdateStructure {
    pub atoms: Vec<Atom>,
}

// System to handle incoming structure updates
pub fn update_crystal_system(
    mut crystal: ResMut<Crystal>,
    mut events: EventReader<UpdateStructure>,
) {
    for event in events.read() {
        crystal.atoms = event.atoms.clone();
    }
}
