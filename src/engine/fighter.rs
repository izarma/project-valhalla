use bevy::prelude::*;
use bevy_ggrs::*;

use crate::multiplayer::session_builder::*;
use crate::engine::fighter_input::*;
use crate::engine::fighter_state_stack::*;
use crate::animations::asset_loader::*;
use crate::animations::fighter_state_animations::*;
use crate::animations::sprite_animations::*;
use crate::GameState;

pub struct AddFighterPlugin;

#[derive(Component)]
#[require(Sprite, SpriteAnimState, PlayerState)]
pub struct Fighter {
    pub handle: usize
}

impl Plugin for AddFighterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<FighterActions>()
        .add_event::<AnimationEvent>()
        .add_systems(Startup, start_matchbox_socket)
        .add_systems(Update, wait_for_players)
        .add_systems(OnEnter(GameState::InGame),(setup_texture_atlases,spawn_fighters).chain())
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, (move_players, fighter_actions_stack, update_fighter_animation).chain());
    
    }
}

fn spawn_fighters(
    mut commands: Commands,
    images: Res<ImageAssets>,
    player_animations: Res<PlayerAnimations>,
) {
    // Player 1
    commands
        .spawn((
            Fighter { 
                handle: 0
            },
            Transform::from_translation(Vec3::new(-2., 0., 0.))
            .with_scale(Vec3::new(0.05, 0.05, 1.)),
            Sprite::from_atlas_image(
                images.idle.clone(),
                TextureAtlas {
                    layout: player_animations.idle.texture_handle.clone(),
                    index: 0,
                },
            ),
            SpriteAnimState {
                start_index: 0,
                end_index: player_animations.idle.frames - 1,
                timer: Timer::from_seconds(1.0/12.0,TimerMode::Repeating),
            }
        ))
        .add_rollback();

    // Player 2
    commands
        .spawn((
            Fighter { 
                handle: 1
            },
            Transform::from_translation(Vec3::new(2., 0., 0.))
            .with_scale(Vec3::new(-0.05, 0.05, 1.)),
            Sprite::from_atlas_image(
                images.idle.clone(),
                TextureAtlas {
                    layout:player_animations.idle.texture_handle.clone(),
                    index: 0,
                },
            ),
            SpriteAnimState {
                start_index: 0,
                end_index: 4,
                timer: Timer::from_seconds(1.0/12.0,TimerMode::Repeating),
            }
        ))
        .add_rollback();
}

