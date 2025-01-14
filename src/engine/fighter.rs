use bevy::prelude::*;
use bevy_ggrs::*;

use crate::animations::sprite_animations::SpriteAnimState;
use crate::multiplayer::session_builder::*;
use crate::engine::fighter_input::*;
use crate::animations::asset_loader::ImageAssets;
use crate::animations::fighter_state_animations::*;
use crate::GameState;

pub struct AddFighterPlugin;

#[derive(Component)]
#[require(Sprite)]
pub struct Fighter {
    pub handle: usize
}

impl Plugin for AddFighterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, start_matchbox_socket)
        .add_systems(Update, wait_for_players)
        .add_systems(OnEnter(GameState::InGame),spawn_fighters)
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_players);
    }
}

fn spawn_fighters(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
) {

    let frame_size = UVec2::new(60, 50);
    // Create TextureAtlasLayouts
    let idle_layout = TextureAtlasLayout::from_grid((frame_size) as UVec2, 5, 1, None, None);
    let idle_layout_handle = texture_atlases.add(idle_layout);

    // Player 1
    commands
        .spawn((
            Fighter { handle: 0 },
            Transform::from_translation(Vec3::new(-2., 0., 0.))
            .with_scale(Vec3::new(0.05, 0.05, 1.)),
            Sprite::from_atlas_image(
                images.idle.clone(),
                TextureAtlas {
                    layout: idle_layout_handle.clone(),
                    index: 0,
                },
            ),
            SpriteAnimState {
                start_index: 0,
                end_index: 4,
                frame_size,
                timer: Timer::from_seconds(1.0/12.0,TimerMode::Repeating),
            }
        ))
        .add_rollback();

    // Player 2
    commands
        .spawn((
            Fighter { handle: 1},
            Transform::from_translation(Vec3::new(2., 0., 0.)),
            Sprite {
                color: Color::srgb(0., 0.4, 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
        ))
        .add_rollback();
}

