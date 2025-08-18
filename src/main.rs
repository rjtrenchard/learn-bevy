use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (
                print_position_system,
                update_position_system,
                print_position_system,
            )
                .chain(),
        )
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        Position {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    ));
    commands.spawn((
        Person,
        Position {
            x: 3.0,
            y: 5.0,
            z: 6.0,
        },
    ));
    commands.spawn((
        Person,
        Position {
            x: 4.0,
            y: 7.0,
            z: 8.0,
        },
    ));
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {} {}", position.x, position.y, position.z);
    }
}

fn update_position_system(mut query: Query<&mut Position, With<Person>>) {
    for mut position in &mut query {
        if position.x == 1.0 {
            position.x = 2.0;
        }
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct Entity(u64);
