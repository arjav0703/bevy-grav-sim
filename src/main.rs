use bevy::prelude::*;
use particular::prelude::*;

const POSITION_SCALE: f32 = 1.0 / 1000.0;
const MASS_SCALE: f64 = 3e18;
const GRAVITATIONAL_CONSTANT: f64 = 2e-14;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((StartupPlugin, InputPlugin))
        .add_systems(
            Update,
            (update_bodies, sync_body_transforms, render_bodies).chain(),
        )
        .run();
}

fn update_bodies(mut bodies: Query<&mut Body>, time: Res<Time>) {
    let dt = 10.0;
    for (acceleration, mut body) in bodies
        .iter()
        .accelerations(&mut sequential::BruteForceScalar)
        .zip(&mut bodies)
    {
        let body = &mut *body;
        println!("{:?}", Vec3::from(acceleration) * dt);
        body.velocity += Vec3::from(acceleration) * dt;
        body.position += body.velocity * dt;
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_bodies));
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut bodies: Query<&mut Body>) {
    if input.pressed(KeyCode::KeyA) {
        for mut body in &mut bodies {
            body.position.x -= 1000.0;
        }
    } else if input.pressed(KeyCode::KeyD) {
        for mut body in &mut bodies {
            body.position.x += 1000.0;
        }
    }
}

#[derive(Particle)]
#[dim(3)]
#[derive(Component)]
struct Body {
    name: String,
    mass: f64,
    mu: f32,
    position: Vec3,
    velocity: Vec3,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_bodies(mut commands: Commands) {
    let earth_mass = 5.924e24;
    let moon_mass = 7.348e22;
    let distance = 334400.0_f64;

    let earth_mu = GRAVITATIONAL_CONSTANT * earth_mass;
    let orbital_speed = (earth_mu / distance).sqrt() as f32;

    let moon_velocity = Vec3::new(0.0, orbital_speed, 0.0);
    let earth_velocity = -moon_velocity * (moon_mass / earth_mass) as f32;

    commands.spawn(Body {
        name: "earth".to_string(),
        mass: earth_mass,
        mu: earth_mu as f32,
        position: Vec3::new(0.0, 0.0, 0.0),
        velocity: earth_velocity,
    });
    commands.spawn(Body {
        name: "moon".to_string(),
        mass: moon_mass,
        mu: (GRAVITATIONAL_CONSTANT * moon_mass) as f32,
        position: Vec3::new(distance as f32, 0.0, 0.0),
        velocity: moon_velocity,
    });
}

fn sync_body_transforms(mut bodies: Query<(&Body, &mut Transform), Changed<Body>>) {
    for (body, mut transform) in &mut bodies {
        transform.translation = body.position * POSITION_SCALE;
    }
}

fn render_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bodies: Query<(Entity, &Body), Added<Body>>,
) {
    for (entity, body) in &bodies {
        let radius = ((body.mass / MASS_SCALE) as f32).cbrt().max(4.0);
        commands.entity(entity).insert((
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_translation(body.position * POSITION_SCALE),
        ));
    }
}
