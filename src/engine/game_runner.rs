use bevy::prelude::*;
use bevy_ggrs::*;
use crate::engine::fighter::AddFighterPlugin;
use crate::multiplayer::config::Config;

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((AddFighterPlugin, GgrsPlugin::<Config>::default()))
        .rollback_component_with_clone::<Transform>();
    }
}