use bevy::{app::AppExit, prelude::*};

use crate::{
    constants::*,
    data::{
        component::{MenuButtonAction, SnakeNode},
        states::{AppState, InGameState},
        PauseStateRes, SnakeType,
    },
};

use crate::data::Direction;

pub fn change_direction_key_event(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut SnakeNode)>,
    pause_state: Res<PauseStateRes>,
    cur_game_state: Res<State<InGameState>>,
) {
    if pause_state.0 == true || *cur_game_state.get() != InGameState::Playing {
        return;
    }
    let mut direction: Option<Direction> = None;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction = Some(Direction::Left);
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction = Some(Direction::Right);
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction = Some(Direction::Up);
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction = Some(Direction::Down);
    }
    if None == direction {
        return;
    }

    let mut vec = Vec::from_iter(query.iter_mut());
    vec.reverse();
    // find the head node
    let head = (&vec)
        .into_iter()
        .find(|item| {
            if let SnakeType::Head(_) = item.1.snake_type {
                return true;
            }
            if SnakeType::Unknown == item.1.snake_type {
                return true;
            }
            return false;
        })
        .unwrap();

    let mut snake_node_after_head_x = 0.;
    let mut snake_node_after_head_y = 0.;

    // find the position of the node after the head
    for snake_node in &vec {
        if SnakeType::Body == snake_node.1.snake_type {
            snake_node_after_head_x = snake_node.0.translation.x;
            snake_node_after_head_y = snake_node.0.translation.y;
            break;
        }
    }

    let dir = direction.take().unwrap();
    // stop change the move direction in this circumstances
    if dir == Direction::Right
        && snake_node_after_head_x == head.0.translation.x + SNAKE_NODE_SIZE
        && snake_node_after_head_y == head.0.translation.y
    {
        return;
    }
    if dir == Direction::Left
        && snake_node_after_head_x == head.0.translation.x - SNAKE_NODE_SIZE
        && snake_node_after_head_y == head.0.translation.y
    {
        return;
    }
    if dir == Direction::Up
        && snake_node_after_head_x == head.0.translation.x
        && snake_node_after_head_y == head.0.translation.y + SNAKE_NODE_SIZE
    {
        return;
    }
    if dir == Direction::Down
        && snake_node_after_head_x == head.0.translation.x
        && snake_node_after_head_y == head.0.translation.y - SNAKE_NODE_SIZE
    {
        return;
    }

    // find the head again, set the moving direction
    if let Some(mut node) = query.iter_mut().last() {
        if let SnakeType::Head(_) = node.1.snake_type {
            node.1.snake_type = SnakeType::Head(dir);
        }
        if SnakeType::Unknown == node.1.snake_type {
            node.1.snake_type = SnakeType::Head(dir);
        }
    }
}

pub fn game_state_key_event(
    keyboard_input: Res<Input<KeyCode>>,
    cur_game_state: ResMut<State<InGameState>>,
    mut pause_state: ResMut<PauseStateRes>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *cur_game_state.get() == InGameState::Playing {
            if pause_state.0 == true {
                pause_state.0 = false;
            } else {
                pause_state.0 = true;
            }
        }
    }
}

pub fn button_click_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    cur_app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<NextState<InGameState>>,
    cur_game_state: ResMut<State<InGameState>>,
    mut exit: EventWriter<AppExit>,
    mut speed: ResMut<Time::<Fixed>>,
) {
    let mut start_game_action = |level: i32| {
        let mut start_game = false;
        if AppState::MainMenu == *cur_app_state.as_ref().get() {
            start_game = true;

            app_state.set(AppState::InGame);
            game_state.set(InGameState::Playing);
        }
        if AppState::InGame == *cur_app_state.as_ref().get() {
            match *cur_game_state.as_ref().get() {
                InGameState::GameOver => {
                    start_game = true;
                    game_state.set(InGameState::Playing);
                }
                _ => {}
            }
        }
        if start_game {
            let new_speed = match level {
                0 => EASY_MODE_FIXED_TIMESTEP,
                1 => NORMAL_MODE_FIXED_TIMESTEP,
                2 => HARD_MODE_FIXED_TIMESTEP,
                _ => NORMAL_MODE_FIXED_TIMESTEP,
            };
            speed.set_timestep_seconds(new_speed);
        }
    };

    let mut exit_game_action = || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            exit.send_default();
        }
    };

    // if the menu is showing, the S and Q can be clicked
    if *cur_app_state.get() == AppState::MainMenu || *cur_game_state.get() == InGameState::GameOver
    {
        if keyboard_input.just_pressed(KeyCode::E) {
            start_game_action(0);
            return;
        }
        if keyboard_input.just_pressed(KeyCode::N) {
            start_game_action(1);
            return;
        }
        if keyboard_input.just_pressed(KeyCode::H) {
            start_game_action(2);
            return;
        }
        if keyboard_input.just_pressed(KeyCode::Q) {
            exit_game_action();
            return;
        }
    }

    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match *action {
                MenuButtonAction::StartGameLevel1 => {
                    start_game_action(0);
                }
                MenuButtonAction::StartGameLevel2 => {
                    start_game_action(1);
                }
                MenuButtonAction::StartGameLevel3 => {
                    start_game_action(2);
                }
                MenuButtonAction::Quit => {
                    exit_game_action();
                }
            }
        }
    }
}
