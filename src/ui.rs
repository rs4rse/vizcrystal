use bevy::prelude::*;
use std::collections::HashMap;

use crate::constants::{get_element_color, get_element_size};
use crate::structure::{AtomEntity, Crystal};

// Component for UI text
#[derive(Component)]
pub(crate) struct FileUploadText;

// System to set up file upload UI
pub fn setup_file_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Drag and drop an XYZ file here to visualize"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        FileUploadText,
    ));
}

// System to update file upload UI
pub fn update_file_ui(
    file_drag_drop: Res<crate::io::FileDragDrop>,
    mut text_query: Query<&mut Text, With<FileUploadText>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        if let Some(ref path) = file_drag_drop.dragged_file {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                // Update the text content
                **text = format!("Loaded: {}", file_name);
            }
        } else {
            **text = "Drag and drop an XYZ file here to visualize".to_string();
        }
    }
}

// System to clear existing atoms when new crystal is loaded
#[allow(dead_code)]
pub fn clear_old_atoms(
    mut commands: Commands,
    atom_query: Query<Entity, With<AtomEntity>>,
) {
    for entity in atom_query.iter() {
        commands.entity(entity).despawn();
    }
}

// Updated setup_scene to handle crystal changes
pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crystal: Res<Crystal>,
) {
    // Only spawn atoms if we have a crystal resource
    spawn_atoms(&mut commands, &mut meshes, &mut materials, &crystal);
    
    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        ..default()
    });
}

// System to respawn atoms when crystal changes
pub fn update_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crystal: Res<Crystal>,
    atom_query: Query<Entity, With<AtomEntity>>,
) {
    if crystal.is_changed() {
        // Clear existing atoms
        for entity in atom_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Spawn new atoms
        spawn_atoms(&mut commands, &mut meshes, &mut materials, &crystal);
        
        println!("Scene updated with new crystal structure");
    }
}

// Helper function to spawn atoms
fn spawn_atoms(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    crystal: &Crystal,
) {
    // Create a sphere mesh for atoms
    let sphere_mesh = meshes.add(Sphere::new(1.0));

    // Create materials for different elements
    let mut element_materials: HashMap<String, Handle<StandardMaterial>> = HashMap::new();

    // Spawn atoms as 3D spheres
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
            Transform::from_xyz(atom.x, atom.y, atom.z)
                .with_scale(Vec3::splat(get_element_size(&atom.element))),
            AtomEntity,
        ));
    }
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
            // Attach a directional light to the camera
            parent.spawn((
                DirectionalLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
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
