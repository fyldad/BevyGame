use bevy::math::ops::{cos, sin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_object)
        .run();
}

#[derive(Component)]
struct MovementSpeed(f32);

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
    // Camera
    commands.spawn(Camera2d);

    commands.insert_resource(Angle::default());

}

fn move_object(
    time: Res<Time>,
    // keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &MovementSpeed)>,
    mut angle: ResMut<Angle>,
) {

    // Movable Square
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(LinearRgba::WHITE))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        // Movable,
        MovementSpeed(200.0),
    ));

    for (mut transform, speed) in &mut query {
        let mut direction = Vec3::ZERO;
        angle.move_angle();
        let point = angle.get_point();
        direction.x += point.0;
        direction.y += point.1;

        direction = direction.normalize();
        transform.translation += direction * speed.0 * time.delta_secs();
    }
}
