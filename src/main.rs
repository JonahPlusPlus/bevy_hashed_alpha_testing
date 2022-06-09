use bevy::{prelude::*, input::mouse::MouseMotion};
use rand::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

const MOVE_SPEED : f32 = 20.0;
const ROT_SPEED : f32 = 1.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(transform)
        .add_system(move_camera)
        .run();
}

#[derive(Component)]
struct BoxTag;

#[derive(Component)]
struct CameraTag {
    pub yaw: f32,
    pub pitch: f32
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let mesh = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    
    let texture_handle = asset_server.load("rainbow_clouds.png");

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // generate objects
    for _ in 0..10000 {
        commands.spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(rng.gen::<f32>() * 100.0, rng.gen::<f32>() * 100.0, rng.gen::<f32>() * 100.0),
            ..default()
        }).insert(BoxTag);
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            range: 145.0, // close to sqrt(100^2 + 100^2)
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let transform = Transform::from_xyz(-2.0, -2.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y);
    let euler = transform.rotation.to_euler(EulerRot::XYZ);
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: transform,
        ..default()
    }).insert(CameraTag { yaw: euler.2.to_degrees(), pitch: euler.1.to_degrees() });
}

fn transform(time: Res<Time>, mut query: Query<&mut Transform, With<BoxTag>>) {
    let delta = time.time_since_startup().as_secs_f32().sin() / 2.0;
    for mut transform in query.iter_mut() {
        transform.translation.x += delta;
    }
}

fn move_camera(time: Res<Time>, keys: Res<Input<KeyCode>>, mut motion_evr: EventReader<MouseMotion>, mut query: Query<(&mut Transform, &mut CameraTag)>) {
    let mut delta = Vec2::ZERO;
    for ev in motion_evr.iter() {
        delta += ev.delta;
    }
    
    let axial = (keys.pressed(KeyCode::W) as u8 as f32) - (keys.pressed(KeyCode::S) as u8 as f32);
    let lateral = (keys.pressed(KeyCode::D) as u8 as f32) - (keys.pressed(KeyCode::A) as u8 as f32);
    
    for (mut transform, mut camera) in query.iter_mut() {
        camera.yaw -= delta.x * ROT_SPEED * time.delta_seconds();
        camera.pitch += delta.y * ROT_SPEED * time.delta_seconds();

        camera.pitch = camera.pitch.clamp(-89.0, 89.9);

        let yaw_radians = camera.yaw.to_radians();
		let pitch_radians = camera.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians) * Quat::from_axis_angle(-Vec3::X, pitch_radians);

        let forward = transform.forward().clone();
        let right = transform.right().clone();
        transform.translation +=  forward * axial * MOVE_SPEED * time.delta_seconds();
        transform.translation +=  right * lateral * MOVE_SPEED * time.delta_seconds();
    }
}
