use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StartupPlugin)
        .add_systems(Update, print_body_info)
        .run();
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_bodies));
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
