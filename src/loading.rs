use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading).add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Game)
                .with_collection::<TextureAssets>()
                .with_collection::<FontAssets>(),
        );
    }
}
#[derive(AssetCollection)]
pub struct TextureAssets {
    //#[asset(path = "bevy.png")]
    //pub texture: Handle<Image>,
    #[asset(path = "textures/spaceship.png")]
    pub spaceship: Handle<Image>,
    #[asset(path = "textures/fuel.png")]
    pub fuel: Handle<Image>,
    #[asset(path = "textures/grass.png")]
    pub grass: Handle<Image>,
}
#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Silkscreen-Bold.ttf")]
    pub silkscreen_bold: Handle<Font>,
    #[asset(path = "fonts/Silkscreen-Regular.ttf")]
    pub silkscreen: Handle<Font>,
}
