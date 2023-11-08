use bevy::prelude::*;
use bevy::window::Window;
use crate::data::component::{OnMainMenuScreen, MenuButtonAction};


pub fn enter_game_over_menu_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
) {
    let window = windows.single();
    let window_width = window.resolution.width();
    let node_width: f32 = 500.;
    let node_height: f32 = 500.;

    let padding_left = (window_width - node_width) / 2.0;

    let title_color = Color::rgba_u8(117, 15, 127, 180);
    spawn_menu(
        &mut commands,
        asset_server,
        padding_left,
        node_width,
        node_height,
        title_color,
        "Game Over",
    );
}

pub fn exit_game_over_menu_system(
    mut commands: Commands,
    query: Query<Entity, With<OnMainMenuScreen>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn exit_main_menu_system(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn enter_main_menu_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
) {
    let window = windows.single();
    let window_width = window.resolution.width();
    let node_width: f32 = 500.;
    let node_height: f32 = 500.;

    let padding_left = (window_width - node_width) / 2.0;

    let title_color = Color::rgba_u8(147, 215, 197, 255);
    spawn_menu(
        &mut commands,
        asset_server,
        padding_left,
        node_width,
        node_height,
        title_color,
        "Snake",
    );
}

pub fn spawn_menu(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    padding_left: f32,
    node_width: f32,
    node_height: f32,
    title_color: Color,
    title_content: &str,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(node_width),
                    height: Val::Px(node_height),
                    left: Val::Px(padding_left),
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(node_width),
                        height: Val::Px(node_height),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(6.),
                        ..default()
                    },
                    background_color: Color::rgba_u8(41, 147, 134, 200).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            title_content,
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 60.0,
                                color: title_color,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::StartGameLevel1,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "(E) Easy",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::rgba_u8(136, 190, 200, 187),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::StartGameLevel2,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "(N) Normal",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::rgba_u8(136, 190, 200, 187),
                                },
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::StartGameLevel3,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "(H) Hard",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::rgba_u8(136, 190, 200, 187),
                                },
                            ));
                        });

                    // 退出按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(50.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "(Q) Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::rgba_u8(136, 190, 200, 187),
                                },
                            ));
                        });
                });
        });
}