use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading).add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Game)
                .with_collection::<TextureAssets>(),
        );
    }
}
#[derive(AssetCollection)]
pub struct TextureAssets {
    //#[asset(path = "bevy.png")]
    //pub texture: Handle<Image>,
}
