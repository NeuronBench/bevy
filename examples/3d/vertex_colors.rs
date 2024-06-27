//! Illustrates the use of vertex colors.

use bevy::{prelude::*, render::mesh::VertexAttributeValues};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, step)
        .run();
}

#[derive(Resource)]
struct MyCubeHandle(Handle<Mesh>);

#[derive(Resource)]
struct Time(f32);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // cube
    // Assign vertex colors based on vertex positions
    let mut colorful_cube = Mesh::from(Cuboid::default());
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        colorful_cube.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
            .collect();
        colorful_cube.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    let cube_handle = meshes.add(colorful_cube);
    commands.spawn(PbrBundle {
        mesh: cube_handle.clone(),
        // This is the default color, but note that vertex colors are
        // multiplied by the base color, so you'll likely want this to be
        // white if using vertex colors.
        material: materials.add(Color::srgb(1., 1., 1.)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(MyCubeHandle(cube_handle));
    commands.insert_resource(Time(0.0));
}


fn step(
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut my_cube: ResMut<MyCubeHandle>
) {
    *time = Time((*time).0 + 0.015);
    if let Some(mesh) = meshes.get_mut(&my_cube.0) {
        mesh.remove_attribute(Mesh::ATTRIBUTE_COLOR);
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[r, g, b]| [time.0.cos() * (1. - *r) / 2.,
                                  time.0.cos() * ((time.0 * 5.0).sin() - *g) / 2.,
                                  time.0.cos() * (1. - *b) / 2., 1.])
                .collect();
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
    }
}
