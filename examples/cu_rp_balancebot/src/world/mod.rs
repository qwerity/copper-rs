use avian3d::prelude::*;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::Skybox;
use bevy::input::{
    keyboard::KeyCode,
    mouse::{MouseButton, MouseMotion, MouseWheel},
};
use bevy::math::DVec3;
use bevy::pbr::{
    DefaultOpaqueRendererMethod, ScreenSpaceReflectionsBundle, ScreenSpaceReflectionsSettings,
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[cfg(feature = "sim-embed")]
mod res {
    pub const BALANCEBOT: &str = "embedded://balancebot_sim/world/assets/balancebot.glb";
    pub const SKYBOX: &str = "embedded://balancebot_sim/world/assets/skybox.ktx2";
    pub const DIFFUSE_MAP: &str = "embedded://balancebot_sim/world/assets/diffuse_map.ktx2";
}

#[cfg(not(feature = "sim-embed"))]
mod res {
    pub const BALANCEBOT: &str = "src/assets/balancebot.glb";
    pub const SKYBOX: &str = "src/assets/skybox.ktx2";
    pub const DIFFUSE_MAP: &str = "src/assets/diffuse_map.ktx2";
}
const TABLE_HEIGHT: f32 = 0.724;
const RAIL_WIDTH: f64 = 0.55; // 55cm
const RAIL_HEIGHT: f64 = 0.02;
const RAIL_DEPTH: f64 = 0.06;

const CART_WIDTH: f64 = 0.040;
const CART_HEIGHT: f64 = 0.045; // The mid rail is 1cm above the bottom rail, and the cart is 35mm tall.
const CART_DEPTH: f64 = RAIL_DEPTH;

const ROD_WIDTH: f64 = 0.007; // 7mm
const ROD_HEIGHT: f64 = 0.50; // 50cm
const ROD_DEPTH: f64 = ROD_WIDTH;

const AXIS_LENGTH: f64 = 0.02;

const STEEL_DENSITY: f64 = 7800.0; // kg/m^3
const ALUMINUM_DENSITY: f64 = 2700.0; // kg/m^3

#[allow(dead_code)]
const ROD_VOLUME: f64 = ROD_WIDTH * ROD_HEIGHT * ROD_DEPTH;

#[derive(Resource)]
struct CameraControl {
    rotate_sensitivity: f32,
    zoom_sensitivity: f32,
    move_sensitivity: f32,
}

#[derive(Resource, PartialEq, Eq)]
enum SimulationState {
    Running,
    Paused,
}

#[derive(Component)]
pub struct Cart;

#[derive(Component)]
pub struct Rod;

pub fn build_world(app: &mut App) -> &mut App {
    app.insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPickingPlugins,
            PhysicsPlugins::default().with_length_unit(1000.0),
            // EditorPlugin::default(),
        ))
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .insert_resource(SimulationState::Paused)
        .insert_resource(CameraControl {
            rotate_sensitivity: 0.05,
            zoom_sensitivity: 3.5,
            move_sensitivity: 0.01,
        })
        .insert_resource(Gravity::default())
        .insert_resource(Time::<Physics>::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, setup_entities)
        .add_systems(Update, toggle_simulation_state)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, update_physics)
}

fn chessboard_setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));
    // Chessboard Plane
    let black_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        reflectance: 0.4,
        perceptual_roughness: 0.4,
        ..default()
    });

    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        reflectance: 0.4,
        perceptual_roughness: 0.4,
        ..default()
    });

    for x in -3..4 {
        for z in -3..4 {
            commands.spawn((PbrBundle {
                mesh: plane_mesh.clone(),
                material: if (x + z) % 2 == 0 {
                    black_material.clone()
                } else {
                    white_material.clone()
                },
                transform: Transform::from_xyz(x as f32 * 2.0, -TABLE_HEIGHT, z as f32 * 2.0),
                ..default()
            },));
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load the skybox
    let skybox_handle = asset_server.load(res::SKYBOX);

    // Fiat Lux
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    // load the scene
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(res::BALANCEBOT)),
        ..default()
    });

    // Spawn the camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-1.0, 0.1, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load(res::DIFFUSE_MAP),
            specular_map: skybox_handle.clone(),
            intensity: 900.0,
        },
        ScreenSpaceReflectionsBundle {
            settings: ScreenSpaceReflectionsSettings {
                perceptual_roughness_threshold: 0.85, // Customize as needed
                thickness: 0.01,
                linear_steps: 128,
                linear_march_exponent: 2.0,
                bisection_steps: 8,
                use_secant: true,
            },
            ..default()
        },
        Fxaa::default(),
    ));

    // add a ground
    chessboard_setup(&mut commands, &mut meshes, &mut materials);
}

// This needs to match an object / parent object name in the GLTF file (in blender this is the object name).
const CART_GLTF_ASSET_NAME: &'static str = "Cart";

fn try_to_find_cart_entity(query: Query<(Entity, &Name), Without<Cart>>) -> Option<Entity> {
    if let Some((cart_entity, _)) = query
        .iter()
        .find(|(_, name)| name.as_str() == CART_GLTF_ASSET_NAME)
    {
        return Some(cart_entity);
    }
    None
}

fn setup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Name), Without<Cart>>,
) {
    // The cart entity will be loaded from the GLTF file, we need to wait for it to be loaded
    let cart_entity = match try_to_find_cart_entity(query) {
        Some(entity) => entity,
        None => return,
    };

    let cart_collider_model = Collider::cuboid(CART_WIDTH, CART_HEIGHT, CART_DEPTH);
    let cart_mass_props =
        MassPropertiesBundle::new_computed(&cart_collider_model, ALUMINUM_DENSITY); // It is a mix of emptiness and motor and steel.. overall some aluminum?

    commands.entity(cart_entity).insert((
        PickableBundle::default(),
        ExternalForce::default(),
        Cart,
        cart_mass_props,
        Dominance(5),
        RigidBody::Dynamic,
        LockedAxes::new()
            .lock_translation_z()
            .lock_translation_y()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x / 500.0; // Only translate along the X-axis
        }),
    ));

    let rail_entity = commands
        .spawn((
            RigidBody::Static, // The rail doesn't move
                               // CollisionLayers::new(0b01, 0b01),
        ))
        .id();

    // Connect the cart to the rail
    commands.spawn(
        PrismaticJoint::new(rail_entity, cart_entity)
            .with_free_axis(DVec3::X) // Allow movement along the X-axis
            .with_compliance(1e-9)
            .with_linear_velocity_damping(100.0)
            .with_angular_velocity_damping(10.0)
            .with_limits(-RAIL_WIDTH / 2.0, RAIL_WIDTH / 2.0)
            .with_local_anchor_1(DVec3::new(0.0, RAIL_HEIGHT / 2.0, 0.0)) // Rail top edge
            .with_local_anchor_2(DVec3::new(0.0, -CART_HEIGHT / 2.0, 0.0)),
    );

    let rod_collider_model = Collider::capsule(ROD_WIDTH / 2.0, ROD_HEIGHT);
    let rod_mass_props = MassPropertiesBundle::new_computed(&rod_collider_model, STEEL_DENSITY); // overall some aluminum?

    let rod_entity = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cylinder::new(ROD_WIDTH as f32 / 2.0, ROD_HEIGHT as f32)),
                // material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    metallic: 0.8,
                    perceptual_roughness: 0.1,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    0.0,
                    ROD_HEIGHT as f32 / 2.0 /*+ RAIL_HEIGHT as f32*/ + CART_HEIGHT as f32,
                    CART_DEPTH as f32 / 2.0 + ROD_DEPTH as f32 / 2.0 + AXIS_LENGTH as f32,
                ), // Start higher than the cart
                ..default()
            },
            PickableBundle::default(),
            rod_mass_props,
            RigidBody::Dynamic,
            LockedAxes::new()
                .lock_translation_z()
                .lock_rotation_y()
                .lock_rotation_x(),
            Rod,
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                let pivot_world = transform.translation
                    + transform.rotation * Vec3::new(0.0, -ROD_HEIGHT as f32 / 2.0, 0.0);
                transform.rotate_around(pivot_world, Quat::from_rotation_z(drag.delta.x / 50.0));
            }),
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(cart_entity, rod_entity)
            .with_compliance(1e-16)
            .with_linear_velocity_damping(10.0)
            .with_angular_velocity_damping(10.0)
            .with_aligned_axis(DVec3::Z) // Align the axis of rotation along the Z-axis
            .with_local_anchor_1(DVec3::new(
                0.0,
                CART_HEIGHT / 2.0,
                CART_DEPTH / 2.0 + ROD_DEPTH / 2.0 + AXIS_LENGTH,
            )) // aim at the center of the rod at the bottom
            .with_local_anchor_2(DVec3::new(0.0, -ROD_HEIGHT / 2.0, 0.0)), // Anchor on the rod (bottom)
    );

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.0, 2.0),
        ..default()
    });
}

/// Winged some type of orbital camera to explore around the robot.
fn camera_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    control: Res<CameraControl>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let mut camera_transform = query.single_mut();
    let focal_point = Vec3::ZERO; // Define the point to orbit around (usually the center of the scene)

    // Calculate the direction vector from the camera to the focal point
    let direction = camera_transform.translation - focal_point;
    let radius = direction.length(); // Distance from the focal point

    // Zoom with scroll
    for ev in scroll_evr.read() {
        let forward = camera_transform.forward(); // Store forward vector in a variable
        let zoom_amount = ev.y * control.zoom_sensitivity * time.delta_seconds();
        camera_transform.translation += forward * zoom_amount;
    }

    // Rotate camera around the focal point with right mouse button + drag
    if mouse_button_input.pressed(MouseButton::Middle) {
        for ev in mouse_motion.read() {
            let yaw = Quat::from_rotation_y(-ev.delta.x * control.rotate_sensitivity);
            let pitch = Quat::from_rotation_x(-ev.delta.y * control.rotate_sensitivity);

            // Apply the rotation to the direction vector
            let new_direction = yaw * pitch * direction;

            // Update the camera position while maintaining the distance from the focal point
            camera_transform.translation = focal_point + new_direction.normalize() * radius;

            // Ensure the camera is always looking at the focal point
            camera_transform.look_at(focal_point, Vec3::Y);
        }
    }

    // Pan camera with middle mouse button + drag
    if mouse_button_input.pressed(MouseButton::Middle) {
        for ev in mouse_motion.read() {
            let right = camera_transform.right();
            let up = camera_transform.up();
            camera_transform.translation += right * -ev.delta.x * control.move_sensitivity;
            camera_transform.translation += up * ev.delta.y * control.move_sensitivity;
        }
    }

    let forward = if keys.pressed(KeyCode::KeyW) {
        camera_transform.forward() * control.move_sensitivity
    } else if keys.pressed(KeyCode::KeyS) {
        camera_transform.back() * control.move_sensitivity
    } else {
        Vec3::ZERO
    };

    let strafe = if keys.pressed(KeyCode::KeyA) {
        camera_transform.left() * control.move_sensitivity
    } else if keys.pressed(KeyCode::KeyD) {
        camera_transform.right() * control.move_sensitivity
    } else {
        Vec3::ZERO
    };

    let vertical = if keys.pressed(KeyCode::KeyQ) {
        Vec3::Y * control.move_sensitivity
    } else if keys.pressed(KeyCode::KeyE) {
        Vec3::NEG_Y * control.move_sensitivity
    } else {
        Vec3::ZERO
    };

    camera_transform.translation += forward + strafe + vertical;
}

// Space to start / stop the simulation
fn toggle_simulation_state(
    mut state: ResMut<SimulationState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *state == SimulationState::Running {
            *state = SimulationState::Paused;
        } else {
            println!("Starting simulation");
            *state = SimulationState::Running;
        }
    }
}

// Pause / Unpause the physics time.
fn update_physics(state: Res<SimulationState>, mut time: ResMut<Time<Physics>>) {
    if *state == SimulationState::Paused {
        time.pause();
        return;
    }
    time.unpause();
}
