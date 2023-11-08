use bevy::prelude::Resource;

pub mod states;
pub mod component;

#[derive(Resource)]
pub struct PauseStateRes(pub bool);

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SnakeType {
    Body,
    Head(Direction),
    Unknown, // this is the snake init state.
}