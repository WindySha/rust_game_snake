use bevy::prelude::Component;

use super::SnakeType;


#[derive(Component)]
pub struct Snake;

#[derive(Component, Debug, Clone, Copy)]
pub struct SnakeNode {
    pub snake_type: SnakeType,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct SnakeLength(pub usize);

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    StartGameLevel1,
    StartGameLevel2,
    StartGameLevel3,
    Quit,
}