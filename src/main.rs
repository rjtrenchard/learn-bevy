use bevy::prelude::*;

fn main() {
    let pos = Position {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    App::new()
        .add_systems(Startup, add_position)
        .add_systems(Update, print_position_system)
        .add_systems(Update, hello_world)
        .add_systems(Update, test)
        .run();
}

fn hello_world() {
    println!("hello world");
    println!("butts");
}

fn test() {
    println!("{:?}", AStruct(12.0));
    println!("{}", AStruct(1.001).0);
}

#[derive(Debug)]
struct AStruct(f32);

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Component)]
struct Person;

fn add_position(mut commands: Commands) {
    commands.spawn(
        ((
            Person,
            Position {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        )),
    );
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {} {}", position.x, position.y, position.z);
    }
}

struct Entity(u64);
