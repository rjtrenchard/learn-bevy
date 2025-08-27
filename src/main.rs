mod states;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Sprite::from_image(
        asset_server.load("kenney_prototypeTextures/PNG/Orange/texture_01.png"),
    ));
}

fn toggle_state(
    state: Res<State<states::AppState>>,
    mut next_state: ResMut<NextStates<states::AppState>>,
) {
    match state.get() {
        states::AppState::Running => next_state.set(states::AppState::Idle),
        states::AppState::Idle => next_state.set(states::AppState::Running),
    }
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.init_state::<states::AppState>();
    }
}
