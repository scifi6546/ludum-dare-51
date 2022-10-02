mod game;
mod loading;

use crate::CursorIcon::Default;
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::texture::ImageSettings,
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
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_plugin(LoadingPlugin)
    .add_plugin(game::GamePlugin);
    app
}
