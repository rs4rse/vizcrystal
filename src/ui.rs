use std::collections::HashMap;

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

use crate::constants::{get_element_color, get_element_size};
use crate::structure::{AtomEntity, Crystal};

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
pub fn setup_scene(
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

// System to set up the camera
pub fn setup_camera(mut commands: Commands, mut toggle_states: ResMut<ToggleStates>) {
    let camera_transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    let initial_translation = camera_transform.translation;
    let initial_rotation = camera_transform.rotation;
    let initial_scale = camera_transform.scale;
    let initial_target = Vec3::ZERO;

    let camera_entity = commands.spawn((Camera3d::default(), camera_transform)).id();

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
            for toggle_id in [ToggleId::LightAttachment, ToggleId::CameraControls] {
                let label = ToggleId::label(toggle_id, toggle_states.get(toggle_id));
                let current_id = toggle_id;
                parent
                    .spawn((
                        Button,
                        Node {
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        ToggleButton { id: current_id },
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(label),
                            TextFont {
                                font: default(),
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            ToggleText { id: current_id },
                        ));
                    });
            }

            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ResetCameraButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Reset Camera"),
                        TextFont {
                            font: default(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

// Handle button interaction: toggle state and update label
pub fn toggle_button_interaction(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor, &ToggleButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut texts: Query<(&ToggleText, &mut Text)>,
    mut toggle_states: ResMut<ToggleStates>,
    mut toggle_events: EventWriter<ToggleEvent>,
) {
    for (interaction, mut background, toggle_button) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                let new_state = toggle_states.toggle(toggle_button.id);
                toggle_events.write(ToggleEvent {
                    id: toggle_button.id,
                    state: new_state,
                });

                for (text_marker, mut text) in &mut texts {
                    if text_marker.id == toggle_button.id {
                        text.0 = ToggleId::label(toggle_button.id, new_state).into();
                    }
                }

                *background = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

// Handle reset button interaction.
pub fn reset_camera_button_interaction(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ResetCameraButton>),
    >,
    camera_entity: Option<Res<MainCameraEntity>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut camera_rig: Option<ResMut<CameraRig>>,
) {
    for (interaction, mut background) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));

                if let (Some(camera_entity), Some(rig)) =
                    (camera_entity.as_deref(), camera_rig.as_deref_mut())
                {
                    if let Ok(mut transform) = camera_query.get_mut(camera_entity.0) {
                        transform.translation = rig.initial_translation;
                        transform.rotation = rig.initial_rotation;
                        transform.scale = rig.initial_scale;
                        rig.target = rig.initial_target;
                        rig.distance = (rig.initial_translation - rig.initial_target)
                            .length()
                            .max(0.5);
                    }
                }
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

// Respond to toggle events by applying the desired world changes
pub fn handle_toggle_events(
    mut toggle_events: EventReader<ToggleEvent>,
    camera_entity: Option<Res<MainCameraEntity>>,
    light_entity: Option<Res<MainLightEntity>>,
    global_light_xforms: Query<&GlobalTransform, With<DirectionalLight>>,
    mut commands: Commands,
) {
    let Some(camera_entity) = camera_entity else {
        return;
    };
    let Some(light_entity) = light_entity else {
        return;
    };

    for event in toggle_events.read() {
        match event.id {
            ToggleId::LightAttachment => {
                if event.state {
                    // Re-attach to camera; use default local transform so light follows camera orientation.
                    commands
                        .entity(light_entity.0)
                        .insert(ChildOf(camera_entity.0))
                        .insert(Transform::default());
                } else if let Ok(global_transform) = global_light_xforms.get(light_entity.0) {
                    let (scale, rotation, translation) =
                        global_transform.to_scale_rotation_translation();
                    commands.entity(light_entity.0).remove::<ChildOf>();
                    commands.entity(light_entity.0).insert(Transform {
                        translation,
                        rotation,
                        scale,
                    });
                } else {
                    commands.entity(light_entity.0).remove::<ChildOf>();
                }
            }
            ToggleId::CameraControls => {
                // No immediate world action; camera system reads the toggle state directly.
            }
        }
    }
}

// Simple camera controls
pub fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
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
