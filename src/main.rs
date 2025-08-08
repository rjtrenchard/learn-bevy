use bevy::prelude::*;

fn main() {
    App::new()
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
