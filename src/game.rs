use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::Window;
use rand::Rng;

use crate::constants::{BOUNDARY_HEIGHT, BOUNDARY_WIDTH, SNAKE_NODE_SIZE};
use crate::data::component::{Food, SnakeLength, SnakeNode};
use crate::data::Direction;
use crate::data::{PauseStateRes, SnakeType};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_boundary(mut commands: Commands, windows: Query<&mut Window>) {
    let window = windows.single();
    let window_width = window.resolution.width();
    let window_height = window.resolution.height();

    let bg_color = Color::rgba(0., 0.5, 0.25, 0.12);
    let width = BOUNDARY_WIDTH;
    let height: f32 = BOUNDARY_HEIGHT;
    let border_width: f32 = 4.;
    let top_margin = (window_height - height) / 2.0;
    let left_margin = (window_width - width) / 2.0;
    commands.spawn(NodeBundle {
        z_index: ZIndex::Global(-1),
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(border_width)),
            margin: UiRect {
                left: Val::Px(left_margin),
                top: Val::Px(top_margin),
                ..Default::default()
            },
            // align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: bevy::prelude::BackgroundColor(bg_color),
        border_color: Color::WHITE.with_a(0.5).into(),
        ..Default::default()
    });
}

pub fn dismiss_snake_and_food(
    mut commands: Commands,
    snake_query: Query<Entity, With<SnakeNode>>,
    food_query: Query<Entity, With<Food>>,
    score_query: Query<Entity, With<SnakeLength>>,
) {
    for entity in &snake_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &food_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &score_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_snake_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    // let window_width = windows.single().resolution.width();
    let window_height = windows.single().resolution.height();

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::rgba_u8(119, 195, 200, 184),
                },
            )
            .with_alignment(TextAlignment::Left),
            transform: Transform {
                translation: Vec3::new(0., (window_height - 110.) / 2., 0.),
                ..default()
            },
            text_anchor: Anchor::Center,
            ..default()
        },
        SnakeLength(0),
    ));
}

// fn is_not_pause_state() -> impl FnMut(Res<PauseStateRes>) -> bool + Clone {
//     move |pause_state: Res<PauseStateRes>| pause_state.0 == false
// }
pub fn is_not_pause_state(pause_state: Res<PauseStateRes>) -> bool {
    pause_state.0 == false
}

pub fn setup_snake_and_food(mut commands: Commands) {
    let snake_default_pox_x = 0.;
    let snake_default_pox_y = 0.;

    commands
        .spawn((
            create_snake_node_bundle(snake_default_pox_x, snake_default_pox_y),
            SnakeNode {
                // If no need to move when entering the game, snake_type can be set to: SnakeType::Unknown,
                snake_type: SnakeType::Head(Direction::Down),
            },
        ))
        .with_children(|parent| {
            parent.spawn(create_snake_node_child_bundle());
        });

    commands.spawn((
        create_food_bundle(&vec![(snake_default_pox_x, snake_default_pox_y)]),
        Food,
    ));
}

pub fn create_snake_node_bundle(x: f32, y: f32) -> SpriteBundle {
    let border_color = Color::rgba(0.7, 0.34, 0.85, 0.5);
    SpriteBundle {
        sprite: Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(SNAKE_NODE_SIZE, SNAKE_NODE_SIZE)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(x, y, 0.),
            ..default()
        },
        ..default()
    }
}

pub fn create_food_bundle(excepted_position: &Vec<(f32, f32)>) -> SpriteBundle {
    let max_x = (BOUNDARY_WIDTH / 2. / SNAKE_NODE_SIZE) as i32;
    let max_y = (BOUNDARY_HEIGHT / 2. / SNAKE_NODE_SIZE) as i32;

    let mut rng = rand::thread_rng();
    let mut x_rand: i32;
    let mut y_rand: i32;

    'outer: loop {
        x_rand = rng.gen_range(-max_x..=max_x);
        y_rand = rng.gen_range(-max_y..=max_y);

        for &pos in excepted_position {
            if (
                (x_rand as f32) * SNAKE_NODE_SIZE,
                (y_rand as f32) * SNAKE_NODE_SIZE,
            ) == pos
            {
                continue 'outer;
            }
        }
        break 'outer;
    }

    SpriteBundle {
        sprite: Sprite {
            color: Color::rgba_u8(200, 40, 28, 255),
            custom_size: Some(Vec2::new(SNAKE_NODE_SIZE - 4., SNAKE_NODE_SIZE - 4.)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(
                x_rand as f32 * SNAKE_NODE_SIZE,
                y_rand as f32 * SNAKE_NODE_SIZE,
                0.,
            ),
            ..default()
        },
        ..default()
    }
}

pub fn create_snake_node_child_bundle() -> SpriteBundle {
    let bg_color = Color::rgb(0.25, 0.25, 0.75);
    let border_widh = 4.;
    SpriteBundle {
        sprite: Sprite {
            color: bg_color,
            custom_size: Some(Vec2::new(
                SNAKE_NODE_SIZE - border_widh,
                SNAKE_NODE_SIZE - border_widh,
            )),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.1),
            ..default()
        },
        ..default()
    }
}
