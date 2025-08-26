use bevy::prelude::*;

mod states {
    use bevy::state::state::States;

    #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum AppState {
        Idle,
        Running,
    }
}
