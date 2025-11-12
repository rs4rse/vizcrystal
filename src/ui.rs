use bevy::prelude::*;

use std::collections::HashMap;

use crate::constants::{get_element_color, get_element_size};
use crate::structure::{AtomEntity, Crystal, Molecule};

// System to set up the 3D scene
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    molecule: Option<Res<Molecule>>,
    crystal: Option<Res<Crystal>>,
) {
    // Create a sphere mesh for atoms
    let sphere_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));

    // Create materials for different elements
    let mut element_materials: HashMap<String, Handle<StandardMaterial>> = HashMap::new();

    // Spawn atoms as 3D spheres
    if let Some(molecule) = molecule {
        for atom in &molecule.atoms {
            // Get or create material for this element
            let material = element_materials
                .entry(atom.element.clone())
                .or_insert_with(|| {
                    materials.add(StandardMaterial {
                        base_color: get_element_color(&atom.element),
                        metallic: 0.0,
                        ..default()
                    })
                })
                .clone();

            // Spawn the atom as a sphere
            commands.spawn((
                Mesh3d(sphere_mesh.clone()),
                MeshMaterial3d(material),
                Transform {
                    translation: Vec3::new(atom.x, atom.y, atom.z),
                    scale: Vec3::splat(get_element_size(&atom.element)),
                    ..default()
                },
                AtomEntity,
            ));
        }
    }

    // Spawn atoms as 3D spheres
    if let Some(crystal) = crystal {
        for atom in &crystal.atoms {
            // Get or create material for this element
            let material = element_materials
                .entry(atom.element.clone())
                .or_insert_with(|| {
                    materials.add(StandardMaterial {
                        base_color: get_element_color(&atom.element),
                        metallic: 0.0,
                        ..default()
                    })
                })
                .clone();

            // Spawn the atom as a sphere
            commands.spawn((
                Mesh3d(sphere_mesh.clone()),
                MeshMaterial3d(material),
                Transform {
                    translation: Vec3::new(atom.x, atom.y, atom.z),
                    scale: Vec3::splat(get_element_size(&atom.element)),
                    ..default()
                },
                AtomEntity,
            ));
        }
    }

    // Remove static scene light; lighting will be attached to the camera in setup_camera

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        affects_lightmapped_meshes: false,
    });
}

// System to set up the camera
pub fn setup_camera(mut commands: Commands) {
    // Spawn camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .with_children(|parent| {
            // Attach a directional light to the camera so it always points where the camera looks
            // For directional lights, only rotation matters; translation is ignored
            parent.spawn((
                DirectionalLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::default(), // inherit camera rotation; light points along -Z in local space
            ));
        });
}

// Simple camera controls
pub fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let mut rotation = Vec3::ZERO;
        let rotation_speed = 1.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation.y += rotation_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation.y -= rotation_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            rotation.x += rotation_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            rotation.x -= rotation_speed * time.delta_secs();
        }

        // Apply rotation around the center
        if rotation != Vec3::ZERO {
            let distance = transform.translation.length();
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, 0.0),
            );
            transform.translation = transform.translation.normalize() * distance;
            transform.look_at(Vec3::ZERO, Vec3::Y);
        }

        // Zoom controls
        let zoom_speed = 5.0;
        if keyboard_input.pressed(KeyCode::Equal) {
            transform.translation *= 1.0 - zoom_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::Minus) {
            transform.translation *= 1.0 + zoom_speed * time.delta_secs();
        }
    }
}
