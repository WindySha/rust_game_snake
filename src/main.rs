use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use constants::*;
use data::{
    states::{AppState, InGameState},
    PauseStateRes,
};
use game::{
    dismiss_snake_and_food, is_not_pause_state, setup_boundary, setup_camera, setup_snake_and_food,
    setup_snake_score,
};
use interaction::*;
use menu::{
    enter_game_over_menu_system, enter_main_menu_system, exit_game_over_menu_system,
    exit_main_menu_system,
};
use update::{should_eat_food, snake_move};

mod constants;
mod data;
mod game;
mod interaction;
mod menu;
mod update;

pub(crate) struct SnakePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(SnakePlugin {})
        .run();
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let background_color: Color = Color::hex("737376").unwrap();
        app.add_state::<AppState>()
            .add_state::<InGameState>()
            .insert_resource(PauseStateRes(false))
            .insert_resource(ClearColor(background_color))
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(AppState::InGame), (setup_boundary,))
            .add_systems(OnEnter(InGameState::GameOver), enter_game_over_menu_system)
            .add_systems(OnExit(InGameState::GameOver), exit_game_over_menu_system)
            .add_systems(
                OnEnter(InGameState::Playing),
                (
                    dismiss_snake_and_food,
                    apply_deferred,
                    setup_snake_score,
                    setup_snake_and_food,
                )
                    .chain(),
            )
            .add_systems(OnEnter(AppState::MainMenu), enter_main_menu_system)
            .add_systems(OnExit(AppState::MainMenu), exit_main_menu_system)
            .add_systems(
                FixedUpdate,
                (should_eat_food, apply_deferred, snake_move)
                    .chain()
                    .run_if(is_not_pause_state.and_then(
                        in_state(AppState::InGame).and_then(in_state(InGameState::Playing)),
                    )),
            )
            .add_systems(
                Update,
                (
                    change_direction_key_event,
                    game_state_key_event,
                    button_click_system,
                ),
            )
            .insert_resource(Time::<Fixed>::from_seconds(NORMAL_MODE_FIXED_TIMESTEP));

        #[cfg(not(target_arch = "wasm32"))] {
            app.add_systems(Update, bevy::window::close_on_esc);
        }
        
    }
}
