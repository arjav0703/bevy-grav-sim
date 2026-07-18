use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((StartupPlugin, InputPlugin))
        .add_systems(Update, print_body_info)
        .run();
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

fn handle_input(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut commands: Commands) {
    if input.pressed(KeyCode::KeyA) {
        commands.spawn(Body {
            name: "body".to_string(),
            mass: 1e24,
            position: Position { x: 10.0, y: 10.0 },
        });
    }
}

#[derive(Component)]
struct Body {
    name: String,
    mass: f64,
    position: Position,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("Hello world!"),
        TextFont {
            font_size: FontSize::Px(120.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
    ));
}

fn setup_bodies(mut commands: Commands) {
    commands.spawn(Body {
        name: "earth".to_string(),
        mass: 5.924e24,
        position: Position { x: 0.0, y: 0.0 },
    });
    commands.spawn(Body {
        name: "moon".to_string(),
        mass: 7.348e22,
        position: Position {
            x: 384400.0,
            y: 0.0,
        },
    });
}

fn print_body_info(bodies: Query<&Body>) {
    for body in &bodies {
        println!("{} -> {}", body.name, body.mass);
    }
}
