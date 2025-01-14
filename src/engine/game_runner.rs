use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_asset_loader::prelude::*;
use crate::engine::fighter::AddFighterPlugin;
use crate::multiplayer::config::Config;
use crate::animations::asset_loader::ImageAssets;
use crate::animations::sprite_animations::animate_sprite;
use crate::GameState;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Matchmaking),
        )
        .add_plugins((AddFighterPlugin, GgrsPlugin::<Config>::default()))
        .add_systems(Update, animate_sprite)
        .rollback_component_with_clone::<Transform>();
    }
}