use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::animations::fighter_state_animations::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "Fighter/idle.png")]
    pub idle: Handle<Image>,
    #[asset(path = "Fighter/attack.png")]
    pub attack: Handle<Image>,
    #[asset(path = "Fighter/special.png")]
    pub special: Handle<Image>,
    #[asset(path = "Fighter/guard.png")]
    pub guard: Handle<Image>,
}

pub fn setup_texture_atlases(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Define the frame size
    let frame_size = UVec2::new(60, 50);

    // Define the frame counts for each animation
    let idle_frames:usize  = 5;   // Number of frames in idle animation
    let attack_frames:usize = 5; // Number of frames in attack animation
    let special_frames:usize = 5;   // Number of frames in jump animation
    let guard_frames:usize = 2;    // Number of frames in run animation

    // Create texture atlases for each animation
    let idle_atlas_handle = texture_atlases.add(TextureAtlasLayout::from_grid((frame_size) as UVec2, 5, 1, None, None));
    let attack_atlas_handle =  texture_atlases.add(TextureAtlasLayout::from_grid((frame_size) as UVec2, 5, 1, None, None));
    let special_atlas_handle =  texture_atlases.add(TextureAtlasLayout::from_grid((frame_size) as UVec2, 2, 3, None, None));
    let guard_atlas =  texture_atlases.add(TextureAtlasLayout::from_grid((frame_size) as UVec2, 1, 2, None, None));

    commands.insert_resource(PlayerAnimations {
        idle: Animation {
            frames: idle_frames,
            frame_size,
            texture_handle: idle_atlas_handle,
        },
        attack: Animation {
            frames: attack_frames,
            frame_size,
            texture_handle: attack_atlas_handle,
        },
        special: Animation {
            frames: special_frames,
            frame_size,
            texture_handle: special_atlas_handle,
        },
        guard: Animation {
            frames: guard_frames,
            frame_size,
            texture_handle: guard_atlas,
        },
    });
}