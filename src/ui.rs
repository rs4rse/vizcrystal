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
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let mut rotation = Vec3::ZERO;
        let keyboard_mode = toggle_states.get(ToggleId::CameraControls);

        if keyboard_mode {
            // Drain motion events so they don't accumulate when switching modes later.
            for _ in mouse_motion_events.read() {}
            for _ in mouse_wheel_events.read() {}

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
        } else {
            let mut mouse_delta = Vec2::ZERO;
            for motion in mouse_motion_events.read() {
                mouse_delta += motion.delta;
            }

            if mouse_buttons.pressed(MouseButton::Left) {
                let sensitivity = 0.005;
                rotation.y -= mouse_delta.x * sensitivity;
                rotation.x -= mouse_delta.y * sensitivity;
            }

            let mut scroll_total = 0.0;
            for wheel in mouse_wheel_events.read() {
                scroll_total += wheel.y;
            }

            if scroll_total.abs() > f32::EPSILON {
                let zoom_factor = 1.0 - scroll_total * 0.1;
                transform.translation *= zoom_factor.clamp(0.2, 2.0);
            }
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
        if keyboard_mode {
            let zoom_speed = 5.0;
            if keyboard_input.pressed(KeyCode::Equal) {
                transform.translation *= 1.0 - zoom_speed * time.delta_secs();
            }
            if keyboard_input.pressed(KeyCode::Minus) {
                transform.translation *= 1.0 + zoom_speed * time.delta_secs();
            }
        }
    }
}
