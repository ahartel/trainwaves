use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    gizmos::gizmos::Gizmos,
    math::{
        cubic_splines::{CubicCardinalSpline, CubicGenerator},
        primitives::Circle,
        vec2,
    },
    mesh::{Mesh, Mesh2d},
    prelude::Camera2d,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_bezier)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Setup system running!");

    let circle = meshes.add(Circle::new(50.0));
    let color = Color::hsl(360. * 0 as f32, 0.95, 0.7);
    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn(Camera2d);
}

fn draw_bezier(mut gizmos: Gizmos) {
    let points = vec![
        vec2(-500., -200.),
        vec2(-250., 250.),
        vec2(250., 250.),
        vec2(500., -200.),
    ];

    let curve = CubicCardinalSpline::new_catmull_rom(points)
        .to_curve()
        .unwrap();
    // Scale resolution with curve length so it doesn't degrade as the length
    // increases.
    let resolution = 100 * curve.segments().len();
    gizmos.linestrip(
        curve.iter_positions(resolution).map(|pt| pt.extend(0.0)),
        Color::srgb(1.0, 1.0, 1.0),
    );

    // Draw a blue circle in the center
    // gizmos.circle_2d(vec2(0.0, 0.0), 50.0, Color::srgb(0.0, 0.0, 1.0));
}
