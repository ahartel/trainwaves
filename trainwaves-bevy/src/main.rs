use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    color::Color,
    ecs::system::Commands,
    gizmos::gizmos::Gizmos,
    math::{
        cubic_splines::{CubicCardinalSpline, CubicGenerator},
        vec2,
    },
    prelude::Camera2d,
};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_bezier)
        .run();
}

fn setup(mut commands: Commands) {
    println!("Setup system running!");

    commands.spawn(Camera2d);
}

fn draw_bezier(mut gizmos: Gizmos) {
    let points = vec![
        vec2(-500., -200.),
        vec2(-250., 250.),
        vec2(250., 250.),
        vec2(500., -200.),
    ];

    let curve = CubicCardinalSpline::new_catmull_rom(points).to_curve().unwrap();
    // Scale resolution with curve length so it doesn't degrade as the length
    // increases.
    let resolution = 100 * curve.segments().len();
    gizmos.linestrip(
        curve.iter_positions(resolution).map(|pt| pt.extend(0.0)),
        Color::srgb(1.0, 1.0, 1.0),
    );
}
