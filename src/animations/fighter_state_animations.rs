use bevy::prelude::*;
use crate::engine::fighter_state_stack::*;
use crate::engine::fighter::*;
use crate::animations::sprite_animations::*;
use crate::animations::asset_loader::*;
use crate::engine::fighter_state_stack::PlayerState;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub attack: Animation,
    pub special: Animation,
    pub guard: Animation,
}

pub struct Animation {
    pub frames: usize,
    pub frame_size: UVec2,
    pub texture_handle: Handle<TextureAtlasLayout>,
}

pub fn update_fighter_animation(
    mut players: Query<(
        &Fighter,
        &PlayerState,
        &mut Sprite,
        &mut SpriteAnimState,
    ),
    Changed<PlayerState>>,
    images: Res<ImageAssets>,
    player_animations: Res<PlayerAnimations>,
) {
    for (fighter, state,  mut sprite, mut sprite_anim_state) in players.iter_mut() {
        let animation = match state.current_state() {
            PlayerStateKind::Idle => &player_animations.idle,
            PlayerStateKind::Attack => &player_animations.attack,
            PlayerStateKind::Special => &player_animations.special,
            PlayerStateKind::Guard => &player_animations.guard,
        };
        // Update the sprite to use the correct texture handle
        sprite.custom_size = Some(animation.frame_size.as_vec2());
        sprite_anim_state.start_index = 0;
        sprite_anim_state.end_index = animation.frames - 1;
        sprite.texture_atlas = Some(TextureAtlas {
            layout: animation.texture_handle.clone(),
            index: 0,
        });

        // Choose the correct image for the sprite
        sprite.image = match state.current_state() {
            PlayerStateKind::Idle => images.idle.clone(),
            PlayerStateKind::Attack => images.attack.clone(),
            PlayerStateKind::Special => images.special.clone(),
            PlayerStateKind::Guard => images.guard.clone(),
        };

        // Debugging info for verification
        // info!(
        //     "Player {} updated to {:?} animation with {} frames",
        //     fighter.handle,
        //     state.current_state(),
        //     animation.frames
        // );
    }
}