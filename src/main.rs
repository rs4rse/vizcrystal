use anyhow::{Context, Result};
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

// Structure to represent an atom from XYZ file
#[derive(Debug, Clone)]
struct Atom {
    element: String,
    x: f32,
    y: f32,
    z: f32,
}

// Structure to hold our crystal data
#[derive(Resource)]
struct Crystal {
    atoms: Vec<Atom>,
}

// Component to mark atom entities
#[derive(Component)]
struct AtomEntity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_crystal, setup_scene, setup_camera).chain())
        .add_systems(Update, camera_controls)
        .run();
}

// Function to parse XYZ file format
fn parse_xyz_file(filename: &str) -> Result<Vec<Atom>> {
    let contents =
        fs::read_to_string(filename).context(format!("Failed to read file: {}", filename))?;

    let lines: Vec<&str> = contents.lines().collect();

    if lines.len() < 2 {
        return Err(anyhow::anyhow!("XYZ file too short"));
    }

    // First line should contain the number of atoms
    let num_atoms: usize = lines[0]
        .trim()
        .parse()
        .context("Failed to parse number of atoms")?;

    // Second line is a comment (we can skip it)
    // Remaining lines contain atom data

    let mut atoms = Vec::new();

    for (i, line) in lines.iter().skip(2).enumerate() {
        if i >= num_atoms {
            break;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue; // Skip malformed lines
        }

        let atom = Atom {
            element: parts[0].to_string(),
            x: parts[1].parse().context("Failed to parse x coordinate")?,
            y: parts[2].parse().context("Failed to parse y coordinate")?,
            z: parts[3].parse().context("Failed to parse z coordinate")?,
        };

        atoms.push(atom);
    }

    Ok(atoms)
}

// Get color for different elements
fn get_element_color(element: &str) -> Color {
    match element.to_uppercase().as_str() {
        "H" => Color::srgb(1.0, 1.0, 1.0),     // Hydrogen - white
        "C" => Color::srgb(0.0, 0.0, 0.0),     // Carbon - black
        "N" => Color::srgb(0.0, 0.0, 1.0),     // Nitrogen - blue
        "O" => Color::srgb(1.0, 0.0, 0.0),     // Oxygen - red
        "S" => Color::srgb(1.0, 1.0, 0.0),     // Sulfur - yellow
        "P" => Color::srgb(1.0, 0.65, 0.0),    // Phosphorus - orange
        "CL" => Color::srgb(0.0, 1.0, 0.0),    // Chlorine - green
        "BR" => Color::srgb(0.65, 0.16, 0.16), // Bromine - dark red
        "I" => Color::srgb(0.58, 0.0, 0.58),   // Iodine - purple
        "FE" => Color::srgb(1.0, 0.65, 0.0),   // Iron - orange
        "ZN" => Color::srgb(0.49, 0.50, 0.69), // Zinc - bluish
        _ => Color::srgb(0.5, 0.5, 0.5),       // Default - gray
    }
}

// Get size for different elements (van der Waals radius scaled)
fn get_element_size(element: &str) -> f32 {
    match element.to_uppercase().as_str() {
        "H" => 0.3,   // Hydrogen
        "C" => 0.4,   // Carbon
        "N" => 0.35,  // Nitrogen
        "O" => 0.32,  // Oxygen
        "S" => 0.45,  // Sulfur
        "P" => 0.42,  // Phosphorus
        "CL" => 0.4,  // Chlorine
        "BR" => 0.45, // Bromine
        "I" => 0.5,   // Iodine
        "FE" => 0.4,  // Iron
        "ZN" => 0.35, // Zinc
        _ => 0.35,    // Default
    }
}

// System to load crystal data
fn load_crystal(mut commands: Commands) {
    // Try to load an example XYZ file, or create some default atoms if file doesn't exist
    let atoms = match parse_xyz_file("crystal.xyz") {
        Ok(atoms) => {
            println!("Loaded {} atoms from crystal.xyz", atoms.len());
            atoms
        }
        Err(e) => {
            println!(
                "Could not load crystal.xyz ({}), using default structure",
                e
            );
            // Create a simple water molecule as default
            vec![
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
            ]
        }
    };

    commands.insert_resource(Crystal { atoms });
}

// System to set up the 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crystal: Res<Crystal>,
) {
    // Create a sphere mesh for atoms
    let sphere_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));

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
            Transform {
                translation: Vec3::new(atom.x, atom.y, atom.z),
                scale: Vec3::splat(get_element_size(&atom.element)),
                ..default()
            },
            AtomEntity,
        ));
    }

    // Add a light source
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        affects_lightmapped_meshes: false,
    });
}

// System to set up the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// Simple camera controls
fn camera_controls(
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
