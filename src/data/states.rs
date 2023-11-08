use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum InGameState {
    #[default]
    Preparing,
    Playing,
    GameOver,
}