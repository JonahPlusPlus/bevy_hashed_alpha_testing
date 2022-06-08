use bevy::prelude::*;
use rand::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(transform)
        .run();
}

#[derive(Component)]
struct BoxTag;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    let material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.8, 0.2, 0.9, 0.5),
        alpha_mode: AlphaMode::Hashed,
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
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, -2.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn transform(time: Res<Time>, mut query: Query<&mut Transform, With<BoxTag>>) {
    let delta = time.time_since_startup().as_secs_f32().sin() / 2.0;
    for mut transform in query.iter_mut() {
        transform.translation.x += delta;
    }
}
