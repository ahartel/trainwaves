use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut},
    },
    gizmos::gizmos::Gizmos,
    math::{
        Vec2,
        cubic_splines::{CubicCardinalSpline, CubicCurve, CubicGenerator},
        primitives::Circle,
        vec2,
    },
    mesh::{Mesh, Mesh2d},
    prelude::{Camera2d, Resource, With},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_bezier, move_train))
        .run();
}

#[derive(Component)]
struct Train;

#[derive(Resource)]
struct TrackCurve {
    curve: CubicCurve<Vec2>,
    resolution: usize,
}

fn initialize_track_curve() -> TrackCurve {
    let points = vec![
        vec2(-500., -200.),
        vec2(-250., 250.),
        vec2(250., 250.),
        vec2(500., -200.),
    ];
    let curve = CubicCardinalSpline::new_catmull_rom(points)
        .to_curve()
        .unwrap();
    let resolution = 100 * curve.segments().len();
    TrackCurve { curve, resolution }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = meshes.add(Circle::new(10.0));
    let color = Color::hsl(360. * 0 as f32, 0.95, 0.7);
    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Train,
    ));
    commands.spawn(Camera2d);

    commands.insert_resource(initialize_track_curve());
}

fn draw_bezier(mut gizmos: Gizmos, track_curve: Res<TrackCurve>) {
    gizmos.linestrip(
        track_curve
            .curve
            .iter_positions(track_curve.resolution)
            .map(|pt| pt.extend(0.0)),
        Color::srgb(1.0, 1.0, 1.0),
    );
}

fn move_train(
    time: Res<bevy::time::Time>,
    track_curve: Res<TrackCurve>,
    mut query: bevy::ecs::system::Query<&mut Transform, With<Train>>,
) {
    // Create a triangle wave that goes from 0 to max_t and back to 0
    let cycle_duration = 6.0; // seconds for one complete cycle (forward + backward)
    let time_in_cycle = (time.elapsed_secs() % cycle_duration) / cycle_duration;

    // CubicCurve position takes a value from 0 to number of segments
    let max_t = track_curve.curve.segments().len() as f32;
    let t = if time_in_cycle < 0.5 {
        // Going forward: 0 -> max_t
        time_in_cycle * 2.0 * max_t
    } else {
        // Going backward: max_t -> 0
        (2.0 - time_in_cycle * 2.0) * max_t
    };

    let position = track_curve.curve.position(t);
    for mut transform in query.iter_mut() {
        transform.translation = position.extend(0.0);
    }
}
