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
        app.add_systems(Startup, setup_bodies);
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
