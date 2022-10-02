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
    GameOver,
}
pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

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
    .add_plugin(game::GamePlugin);
    app
}
