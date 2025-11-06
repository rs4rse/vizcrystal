use std::collections::HashMap;

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::view::RenderLayers;

use crate::constants::{get_element_color, get_element_size};
use crate::structure::{AtomEntity, Crystal};

#[derive(Component)]
pub(crate) struct MainCamera;

/// Identifier for a reusable toggle interaction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ToggleId {
    LightAttachment,
    CameraControls,
}

impl ToggleId {
    fn label(self, state: bool) -> &'static str {
        match (self, state) {
            (ToggleId::LightAttachment, true) => "Light: Attached",
            (ToggleId::LightAttachment, false) => "Light: Detached",
            (ToggleId::CameraControls, true) => "Controls: Keyboard",
            (ToggleId::CameraControls, false) => "Controls: Mouse",
        }
    }
}

/// Stores the current on/off state for each toggle.
#[derive(Resource, Default)]
pub struct ToggleStates {
    states: HashMap<ToggleId, bool>,
}

impl ToggleStates {
    pub fn register(&mut self, id: ToggleId, initial_state: bool) {
        self.states.entry(id).or_insert(initial_state);
    }

    pub fn get(&self, id: ToggleId) -> bool {
        self.states.get(&id).copied().unwrap_or(false)
    }

    pub fn toggle(&mut self, id: ToggleId) -> bool {
        let new_state = !self.get(id);
        self.states.insert(id, new_state);
        new_state
    }
}

/// Marks an entity that spawned the main camera.
#[derive(Resource)]
pub struct MainCameraEntity(pub Entity);

/// Marks the primary directional light used for shading.
#[derive(Resource)]
pub struct MainLightEntity(pub Entity);

/// Component identifying a toggle button instance.
#[derive(Component)]
pub struct ToggleButton {
    pub id: ToggleId,
}

/// Component carried by the text to update when a toggle changes.
#[derive(Component)]
pub struct ToggleText {
    pub id: ToggleId,
}

/// Event emitted whenever a toggle switches state.
#[derive(Event)]
pub struct ToggleEvent {
    pub id: ToggleId,
    pub state: bool,
}

/// Stores camera orbit information and the original configuration so it can be restored.
#[derive(Resource)]
pub struct CameraRig {
    pub target: Vec3,
    pub distance: f32,
    pub initial_target: Vec3,
    pub initial_translation: Vec3,
    pub initial_rotation: Quat,
    pub initial_scale: Vec3,
}

/// Button that resets the camera to its original position/orientation.
#[derive(Component)]
pub struct ResetCameraButton;

// System to set up the 3D scene
pub(crate) fn setup_scene(
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

    // Remove static scene light; lighting will be attached to the camera in setup_camera

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        affects_lightmapped_meshes: false,
    });
}

const GIZMO_LAYER: RenderLayers = RenderLayers::layer(1);

// System to set up the camera
pub fn setup_camera(mut commands: Commands, mut toggle_states: ResMut<ToggleStates>, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let viewport_size = UVec2::new(200, 200);
    let bottom_left_y = window.physical_height() - viewport_size.y - 10;
    let viewport_position = UVec2::new(10, bottom_left_y);

    let camera_transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    let initial_translation = camera_transform.translation;
    let initial_rotation = camera_transform.rotation;
    let initial_scale = camera_transform.scale;
    let initial_target = Vec3::ZERO;

    // Spawn cameras
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                ..default()
            },
            Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            RenderLayers::layer(0),
            MainCamera,
        ))
        .id();

    let light_entity = commands
        .spawn((
            DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::default(), // inherit camera rotation; light points along -Z in local space
            ChildOf(camera_entity),
        ))
        .id();

    toggle_states.register(ToggleId::LightAttachment, true);
    toggle_states.register(ToggleId::CameraControls, true);

    commands.insert_resource(MainCameraEntity(camera_entity));
    commands.insert_resource(MainLightEntity(light_entity));
    commands.insert_resource(CameraRig {
        target: initial_target,
        distance: initial_translation.distance(initial_target),
        initial_target,
        initial_translation,
        initial_rotation,
        initial_scale,
    });
}

// Setup minimal UI with a toggle button
pub fn setup_ui(mut commands: Commands, toggle_states: Res<ToggleStates>) {
    // Root node in top-left
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                top: Val::Px(12.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
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
        })
        .with_children(|parent| {
            // GIZMO CAMERA
            parent.spawn((
                Camera3d { ..default() },
                Camera {
                    order: 1,
                    viewport: Some(Viewport {
                        physical_position: viewport_position,
                        physical_size: viewport_size,
                        ..default()
                    }),
                    ..default()
                },
                Transform::default(),
                GlobalTransform::default(),
                GIZMO_LAYER,
            ));
        });
}

pub(crate) fn spawn_axis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut axis = |color: Color,
                    (x, y, z): (f32, f32, f32),
                    (x_, y_, z_): (f32, f32, f32)|
     -> (
        (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform),
        RenderLayers,
    ) {
        let mesh = meshes.add(Mesh::from(Cuboid::new(x, y, z)));
        let material = materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..default()
        });
        (
            (
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_xyz(x_, y_, z_),
            ),
            GIZMO_LAYER, // visible only to axis camera
        )
    };

    let scale = 2.0;
    commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            GIZMO_LAYER,
        ))
        .with_children(|p| {
            p.spawn(axis(
                Srgba::RED.into(),
                (scale * 1., scale * 0.1, scale * 0.1),
                (scale * 1. / 2., 0., 0.),
            )); // +X
            p.spawn(axis(
                Srgba::GREEN.into(),
                (scale * 0.1, scale * 1., scale * 0.1),
                (0., scale * 1. / 2., 0.),
            )); // +Y
            p.spawn(axis(
                Srgba::BLUE.into(),
                (scale * 0.1, scale * 0.1, scale * 1.),
                (0., 0., scale * 1. / 2.),
            )); // +Z
        });
}

// System to refresh atoms when Crystal resource changes
pub fn refresh_atoms_system(
    mut commands: Commands,
    crystal: Res<Crystal>,
    atom_entities: Query<Entity, With<AtomEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Only run when Crystal resource changes
    if !crystal.is_changed() {
        return;
    }

    // Despawn all existing atoms
    for entity in atom_entities.iter() {
        commands.entity(entity).despawn();
    }

    // Respawn with new positions
    let sphere_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));
    let mut element_materials: HashMap<String, Handle<StandardMaterial>> = HashMap::new();

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

// Simple camera controls
pub(crate) fn camera_controls(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    toggle_states: Res<ToggleStates>,
    mut camera_rig: ResMut<CameraRig>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let keyboard_mode = toggle_states.get(ToggleId::CameraControls);
        let mut yaw_delta = 0.0;
        let mut pitch_delta = 0.0;
        let mut zoom_change = 0.0;
        let mut pan_request = Vec2::ZERO;

        const MIN_DISTANCE: f32 = 0.2;
        const MAX_DISTANCE: f32 = 200.0;

        if keyboard_mode {
            // Drain motion events so they don't accumulate when switching modes later.
            for _ in mouse_motion_events.read() {}
            for _ in mouse_wheel_events.read() {}

            let rotation_speed = 1.0;

            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                yaw_delta += rotation_speed * time.delta_secs();
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                yaw_delta -= rotation_speed * time.delta_secs();
            }
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                pitch_delta += rotation_speed * time.delta_secs();
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                pitch_delta -= rotation_speed * time.delta_secs();
            }

            let zoom_speed = 1.5;
            if keyboard_input.pressed(KeyCode::Equal) {
                zoom_change -= zoom_speed * time.delta_secs();
            }
            if keyboard_input.pressed(KeyCode::Minus) {
                zoom_change += zoom_speed * time.delta_secs();
            }
        } else {
            let mut mouse_delta = Vec2::ZERO;
            for motion in mouse_motion_events.read() {
                mouse_delta += motion.delta;
            }

            if mouse_buttons.pressed(MouseButton::Left) {
                let sensitivity = 0.005;
                yaw_delta -= mouse_delta.x * sensitivity;
                pitch_delta -= mouse_delta.y * sensitivity;
            }

            if mouse_buttons.pressed(MouseButton::Right) {
                pan_request = mouse_delta;
            }

            for wheel in mouse_wheel_events.read() {
                zoom_change -= wheel.y * 0.2;
            }
        }

        // Keep camera offset updated relative to target.
        let mut offset = transform.translation - camera_rig.target;
        if offset.length_squared() < f32::EPSILON {
            offset = Vec3::new(0.0, 0.0, camera_rig.distance.max(1.0));
        }

        if yaw_delta != 0.0 || pitch_delta != 0.0 {
            let rotation = Quat::from_euler(EulerRot::XYZ, pitch_delta, yaw_delta, 0.0);
            offset = rotation * offset;
        }

        if pan_request != Vec2::ZERO {
            let distance = offset.length().max(MIN_DISTANCE);
            let forward = (-offset).normalize_or_zero();
            let mut right = forward.cross(Vec3::Y).normalize_or_zero();
            if right.length_squared() < f32::EPSILON {
                right = Vec3::X;
            }
            let up = right.cross(forward).normalize_or_zero();
            let pan_speed = 0.002 * distance;
            let pan_offset = (-pan_request.x * right + pan_request.y * up) * pan_speed;
            camera_rig.target += pan_offset;
        }

        let mut distance = offset.length().max(MIN_DISTANCE);
        if zoom_change != 0.0 {
            let factor = (1.0 + zoom_change).clamp(0.2, 5.0);
            distance = (distance * factor).clamp(MIN_DISTANCE, MAX_DISTANCE);
        }

        let direction = offset.normalize_or_zero();
        offset = if direction.length_squared() > 0.0 {
            direction * distance
        } else {
            Vec3::new(0.0, 0.0, distance)
        };

        transform.translation = camera_rig.target + offset;
        transform.look_at(camera_rig.target, Vec3::Y);
        transform.scale = Vec3::ONE;
        camera_rig.distance = distance;
    }
}
