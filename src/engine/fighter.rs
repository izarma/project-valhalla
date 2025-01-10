use bevy::prelude::*;
use bevy_ggrs::*;

use crate::multiplayer::session_builder::*;
use crate::engine::fighter_input::*;

pub struct AddFighterPlugin;

#[derive(Component)]
pub struct Fighter {
    pub handle: usize
}

impl Plugin for AddFighterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (spawn_players, start_matchbox_socket))
        .add_systems(Update, wait_for_players)
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_players);
    }
}

fn spawn_players(mut commands: Commands) {
    // Player 1
    commands
        .spawn((
            Fighter { handle: 0 },
            Transform::from_translation(Vec3::new(-2., 0., 0.)), // changed
            Sprite {
                color: Color::srgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
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

