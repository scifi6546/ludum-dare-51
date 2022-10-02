use super::{GameEntity, GameState, PlayerCoolingTimer};
use crate::loading::FontAssets;
use crate::KeyCode::V;
use bevy::ecs::schedule::ShouldRun::No;
use bevy::prelude::*;

pub struct UiPlugin;
#[derive(Component)]
struct CoolingOverlay;
#[derive(Component)]
struct GameOverMenuEntity;
#[derive(Component)]
struct ReturnButton;
mod colors {
    use bevy::prelude::*;

    pub const BUTTON_NORMAL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
    pub const BUTTON_HOVERD_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
    pub const BUTTON_CLICKED_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
    pub const SCORE_SIZE: f32 = 50.0;
    pub const FUEL_BAR_SIZE: Val = Val::Px(100.0);
    pub const FUEL_BAR_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
}
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(ui_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game).with_system(ui_run),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(spawn_game_over),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(return_button),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(clear_gameover),
        );
    }
}
fn clear_gameover(
    mut commands: Commands,
    entity_query: Query<Entity, With<GameOverMenuEntity>>,
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}
fn ui_system(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceAround,
                align_content: AlignContent::SpaceBetween,

                ..default()
            },
            color: Color::rgba(1.0, 0.0, 0.0, 0.3).into(),
            ..default()
        })
        .insert(CoolingOverlay)
        .insert(GameEntity)
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        "1000!!!",
                        TextStyle {
                            font: fonts.silkscreen.clone(),
                            font_size: colors::SCORE_SIZE,
                            color: Color::BLACK.into(),
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::FlexEnd,
                        ..default()
                    }),
                )
                .insert(GameEntity);
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), colors::FUEL_BAR_SIZE),
                    ..default()
                },
                color: colors::FUEL_BAR_COLOR.into(),
                ..default()
            });
        });
}
fn ui_run(
    player_query: Query<&PlayerCoolingTimer, ()>,
    mut cooling_query: Query<&mut UiColor, With<CoolingOverlay>>,
) {
    let player = player_query.iter().next();
    if player.is_none() {
        error!("player does not exist");
        return;
    }
    let player = player.unwrap();
    for mut color in cooling_query.iter_mut() {
        *color = Color::rgba(1.0, 0.0, 0.0, player.get_frac_used()).into();
    }
}
fn spawn_game_over(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameOverMenuEntity);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: colors::BUTTON_NORMAL_COLOR.into(),
            ..default()
        })
        .insert(GameOverMenuEntity)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font: fonts.silkscreen_bold.clone(),
                        font_size: 80.0,
                        color: Color::rgb(0.0, 0.0, 0.0).into(),
                    },
                ))
                .insert(GameOverMenuEntity);
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::rgb(0.6, 0.6, 0.6).into(),
                    ..default()
                })
                .insert(ReturnButton)
                .insert(GameOverMenuEntity)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section(
                            "Start New Game",
                            TextStyle {
                                font: fonts.silkscreen.clone(),
                                font_size: 80.0,
                                color: Color::rgb(0.0, 0.0, 0.0).into(),
                            },
                        ))
                        .insert(GameOverMenuEntity);
                });
        });
}
fn return_button(
    mut color_query: Query<(&mut UiColor, &Interaction), With<ReturnButton>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (mut color, interaction) in color_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = colors::BUTTON_CLICKED_COLOR.into();
                game_state.set(GameState::Game).unwrap()
            }
            Interaction::Hovered => *color = colors::BUTTON_HOVERD_COLOR.into(),
            Interaction::None => *color = colors::BUTTON_NORMAL_COLOR.into(),
        }
    }
}
