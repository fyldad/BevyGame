use bevy::math::ops::{cos, sin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ( spawn_dot_every_frame, move_dots, handle_wall_bounce ))
        .run();
}

#[derive(Component)]
struct DotDirection {
    direction: Vec2,
}

#[derive(Resource, Default)]
struct Angle(f32);

impl Angle {

    fn get_point(&self) -> (f32, f32) {
        let x = cos(self.0 * std::f32::consts::PI / 180.0);
        let y = sin(self.0 * std::f32::consts::PI / 180.0);
        (x, y)
    }

    fn move_angle(&mut self) {
        self.0 += 1.0;
        if self.0 >= 360.0 {
            self.0 = 0.0;
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
    commands.insert_resource(Angle::default());
    commands.insert_resource(Bounds::default());

}

#[derive(Resource)]
struct Bounds {
    pub lower_left: Vec2,
    pub upper_right: Vec2,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            lower_left: Vec2::new(-400.0, -300.0),
            upper_right: Vec2::new(400.0, 300.0),
        }
    }
}

fn spawn_dot_every_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut angle: ResMut<Angle>,
) {
    angle.move_angle();
    let point = angle.get_point();

    let direction = Vec2::new(point.0, point.1).normalize();

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(rand::random(), rand::random(), rand::random()))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        DotDirection {direction: direction * 200.0},
    ));
}

fn handle_wall_bounce (
    bounds: Res<Bounds>,
    mut query: Query<(&Transform, &mut DotDirection)>,
) {
    for (transform, mut dot) in &mut query {
        let position = transform.translation.truncate();
        if position.x < bounds.lower_left.x || position.x > bounds.upper_right.x {
            dot.direction.x *= -1.0;
        }
        if position.y < bounds.lower_left.y || position.y > bounds.upper_right.y {
            dot.direction.y *= -1.0;
        }
    }
}

fn move_dots(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &DotDirection)>,
) {
    for (mut transform, dot_direction) in &mut query {
        transform.translation.x += dot_direction.direction.x * time.delta_secs();
        transform.translation.y += dot_direction.direction.y * time.delta_secs();
    }
}
