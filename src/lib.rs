mod game;
mod loading;

use crate::CursorIcon::Default;
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use loading::{LoadingPlugin, TextureAssets};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Game,
}
pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";
#[derive(Component)]
struct PlayerLabel;

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(LoadingPlugin)
    .add_plugin(game::GamePlugin)
    .add_system_set(
        SystemSet::on_enter(GameState::Game).with_system(load_player),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Game).with_system(input_system),
    );
    app
}
fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_position: Query<&mut Transform, With<PlayerLabel>>,
) {
    let player_speed = 1000.0;
    for mut p in player_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            p.translation.x -= time.delta_seconds() * player_speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            p.translation.x += time.delta_seconds() * player_speed;
        }
        if keyboard_input.pressed(KeyCode::W) {
            p.translation.y += time.delta_seconds() * player_speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            p.translation.y -= time.delta_seconds() * player_speed;
        }
    }
}
fn load_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        })
        .insert(PlayerLabel);
}
