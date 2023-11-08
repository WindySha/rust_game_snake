use bevy::prelude::*;

use crate::constants::{SNAKE_NODE_SIZE, BOUNDARY_WIDTH, BOUNDARY_HEIGHT};
use crate::data::SnakeType;
use crate::data::component::{SnakeNode, Food, SnakeLength};
use crate::data::Direction;
use crate::data::states::InGameState;
use crate::game::{create_snake_node_bundle, create_food_bundle, create_snake_node_child_bundle};


pub fn should_eat_food(
    // mut world: &mut World,
    mut commands: Commands,
    mut snake_query: Query<(&mut Transform, &mut SnakeNode)>,
    food_query: Query<(Entity, &mut Transform), (With<Food>, Without<SnakeNode>)>,
) {
    if let Ok(food_trans) = food_query.get_single() {
        let food_x = food_trans.1.translation.x;
        let food_y = food_trans.1.translation.y;

        let mut eat_food = false;
        let mut head_dir = Direction::Up;
        let mut all_snake_node_position: Vec<(f32, f32)> = Vec::new();
        for mut snake_info in &mut snake_query {
            all_snake_node_position.push((snake_info.0.translation.x, snake_info.0.translation.y));
            if snake_info.1.snake_type == SnakeType::Body {
                continue;
            }
            if food_x == snake_info.0.translation.x {
                if let SnakeType::Head(dir) = snake_info.1.snake_type {
                    if dir == Direction::Up
                        && food_y == snake_info.0.translation.y + SNAKE_NODE_SIZE
                    {
                        eat_food = true;
                    }
                }
                if let SnakeType::Head(dir) = snake_info.1.snake_type {
                    if dir == Direction::Down
                        && food_y == snake_info.0.translation.y - SNAKE_NODE_SIZE
                    {
                        eat_food = true;
                    }
                }
            }
            if food_y == snake_info.0.translation.y {
                if let SnakeType::Head(dir) = snake_info.1.snake_type {
                    if dir == Direction::Right
                        && food_x == snake_info.0.translation.x + SNAKE_NODE_SIZE
                    {
                        eat_food = true;
                    }
                }
                if let SnakeType::Head(dir) = snake_info.1.snake_type {
                    if dir == Direction::Left
                        && food_x == snake_info.0.translation.x - SNAKE_NODE_SIZE
                    {
                        eat_food = true;
                    }
                }
            }
            if eat_food {
                if let SnakeType::Head(dir) = snake_info.1.snake_type {
                    head_dir = dir;
                    snake_info.1.snake_type = SnakeType::Body;
                }
            }
            break;
        }
        if eat_food {
            // dismiss the old food
            commands.entity(food_trans.0).despawn();
            // add food to the snake head
            commands
                .spawn((
                    create_snake_node_bundle(food_x, food_y),
                    SnakeNode {
                        snake_type: SnakeType::Head(head_dir),
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(create_snake_node_child_bundle());
                });
            all_snake_node_position.push((food_x, food_y));
            // create new food object
            commands.spawn((create_food_bundle(&all_snake_node_position), Food));
        }
    }
}

pub fn snake_move(
    mut query: Query<(&mut Transform, &SnakeNode)>,
    mut score_query: Query<(&mut Text, &mut SnakeLength)>,
    mut game_state: ResMut<NextState<InGameState>>,
    cur_game_state: ResMut<State<InGameState>>,
) {
    if *cur_game_state.as_ref().get() == InGameState::GameOver {
        return;
    }

    let snake_length = query.iter().size_hint().0;

    // update the score player getted.
    let score_info = &mut score_query.single_mut();
    let len = score_info.1.as_mut();

    if snake_length > len.0 + 1 {
        len.0 += 1;
        assert!(snake_length == len.0 + 1);

        // we have just eat the food, so do not move the snake, only update the socre.
        let text = score_info.0.as_mut();
        let style = text.sections.first().unwrap().style.clone();
        text.sections = vec![TextSection::new(format!("Score: {}", len.0), style)];
        return;
    }

    let mut vec = Vec::from_iter(&mut query.iter_mut());
    vec.reverse();

    let head = (&mut vec)
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

    if SnakeType::Unknown == head.1.snake_type {
        return;
    }

    let mut last_x: f32 = 0.;
    let mut last_y: f32 = 0.;

    // update snake head position
    if let SnakeType::Head(direction) = head.1.snake_type {
        last_x = head.0.translation.x;
        last_y = head.0.translation.y;
        match direction {
            Direction::Down => head.0.translation.y -= SNAKE_NODE_SIZE,
            Direction::Up => head.0.translation.y += SNAKE_NODE_SIZE,
            Direction::Left => head.0.translation.x -= SNAKE_NODE_SIZE,
            Direction::Right => head.0.translation.x += SNAKE_NODE_SIZE,
        }
    }
    let head_translate_x = head.0.translation.x;
    let head_translate_y = head.0.translation.y;

    // update snake body position
    for snake_node in &mut vec {
        if SnakeType::Body == snake_node.1.snake_type {
            let temp_x = snake_node.0.translation.x;
            let temp_y = snake_node.0.translation.y;

            snake_node.0.translation.x = last_x;
            snake_node.0.translation.y = last_y;

            last_x = temp_x;
            last_y = temp_y;
        }
    }

    let boundary_translation_x =
        SNAKE_NODE_SIZE * ((BOUNDARY_WIDTH / 2. / SNAKE_NODE_SIZE) as i32) as f32;
    let boundary_translation_y =
        SNAKE_NODE_SIZE * ((BOUNDARY_HEIGHT / 2. / SNAKE_NODE_SIZE) as i32) as f32;

    // if the snake head hit the boundary, game over.
    if head_translate_x.abs() > boundary_translation_x.abs()
        || head_translate_y.abs() > boundary_translation_y.abs()
    {
        game_state.set(InGameState::GameOver);
        return;
    }
    // if the snake head hit the snake body, game over.
    for snake_node in &vec {
        if SnakeType::Body == snake_node.1.snake_type {
            let temp_x = snake_node.0.translation.x;
            let temp_y = snake_node.0.translation.y;
            if temp_x == head_translate_x && temp_y == head_translate_y {
                info!("Failed !!!!");
                game_state.set(InGameState::GameOver);
                return;
            }
        }
    }
}