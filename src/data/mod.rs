use bevy::prelude::Resource;

pub mod component;
pub mod states;

#[derive(Resource)]
pub struct PauseStateRes {
    pub user_click_pause: bool,
    pub lose_focus_pause: bool,
}

impl PauseStateRes {
    pub fn new(user_click_pause: bool, lose_focus_pause: bool) -> PauseStateRes {
        return PauseStateRes {
            user_click_pause,
            lose_focus_pause,
        };
    }

    pub fn is_pause_state(&self) -> bool {
        return self.user_click_pause || self.lose_focus_pause;
    }
}

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
