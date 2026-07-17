use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup_bodies)
        .add_systems(Update, hello_world)
        .add_systems(Update, print_body_info)
        .run();
}

fn hello_world() {
    println!("hello world!");
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
