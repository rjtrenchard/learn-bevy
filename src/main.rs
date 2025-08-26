use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .run();
}

// fn add_people(mut commands: Commands) {
//     commands.spawn((
//         Person,
//         Position {
//             x: 1.0,
//             y: 2.0,
//             z: 3.0,
//         },
//     ));
//     commands.spawn((
//         Person,
//         Position {
//             x: 3.0,
//             y: 5.0,
//             z: 6.0,
//         },
//     ));
//     commands.spawn((
//         Person,
//         Position {
//             x: 4.0,
//             y: 7.0,
//             z: 8.0,
//         },
//     ));
// }

// fn print_position_system(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Position>) {
//     if !timer.0.tick(time.delta()).just_finished() {
//         return;
//     }

//     for position in &query {
//         println!("position: {} {} {}", position.x, position.y, position.z);
//     }
// }

// fn update_position_system(
//     time: Res<Time>,
//     mut timer: ResMut<GreetTimer>,
//     mut query: Query<&mut Position, With<Person>>,
// ) {
//     if !timer.0.tick(time.delta()).just_finished() {
//         return;
//     }

//     for mut position in &mut query {
//         if position.x == 1.0 {
//             position.x = 2.0;
//         }
//     }
// }

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct Entity(u64);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Sprite::from_image(
        asset_server.load("kenney_prototypeTextures/PNG/Orange/texture_01.png"),
    ));
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
    }
}
