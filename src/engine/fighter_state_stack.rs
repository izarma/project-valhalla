use bevy::prelude::*;
use crate::engine::fighter::*;
use crate::engine::fighter_input::*;
use crate::animations::sprite_animations::*;
use crate::animations::fighter_state_animations::*;

#[derive(Component, Debug)]
pub struct PlayerState(pub Vec<PlayerStateKind>);

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState(vec![PlayerStateKind::Idle])
    }
}

impl PlayerState {
    pub fn current_state(&self) -> PlayerStateKind {
        *self.0.last().unwrap_or(&PlayerStateKind::Idle)
    }
    pub fn push_state(&mut self, new_state: PlayerStateKind) {
        // Remove existing same state if needed
        self.0.retain(|&s| s != new_state);
        // Add new state
        self.0.push(new_state);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerStateKind {
    Idle,
    Attack,
    Special,
    Guard,
}

pub fn fighter_actions_stack(
    mut fighter_actions_event_reader: EventReader<FighterActions>,
    mut fighter_animation_event_reader: EventReader<AnimationEvent>,
    mut fighters: Query<(
        &Fighter,
        &mut PlayerState,
    )>,
) {
    for action in fighter_actions_event_reader.read() {
        for ( fighter, mut fighter_state) in fighters.iter_mut() {
            //player_state.state.0.retain(|s| !matches!(s, PlayerStateKind::Idle));
            match action {
                FighterActions::Attack(handle) => {
                    if *handle == fighter.handle {
                        fighter_state.push_state(PlayerStateKind::Attack);
                        info!("Player {} performed an Attack! {:?}", handle, fighter_state)
                    }
                }
                FighterActions::Special(handle) => {
                    if *handle == fighter.handle {
                        fighter_state.push_state(PlayerStateKind::Special);
                        info!("Player {} performed a Special Attack! {:?}", handle, fighter_state)
                    }
                }
                FighterActions::Guard(handle) => {
                    if *handle == fighter.handle {
                        fighter_state.push_state(PlayerStateKind::Guard);
                        info!("Player {} is Guarding! {:?}", handle, fighter_state)
                    }
                }
                FighterActions::Forward(handle) => {
                    info!("Player {} moved Forward!", handle)
                }
                FighterActions::Backward(handle) => {
                    info!("Player {} moved Backward!", handle)
                }
            }   
        }
    }

    //TODO: differen fighters not handled
    for animation in fighter_animation_event_reader.read() {
        if let AnimationEventKind::Finished = animation.kind {
            for (fighter, mut fighter_state) in fighters.iter_mut() {
                if let PlayerStateKind::Attack | PlayerStateKind::Special = fighter_state.current_state() {
                    // Pop the Attack state from the stack
                    fighter_state.0.pop();
                    if fighter_state.0.is_empty() {
                        fighter_state.0.push(PlayerStateKind::Idle);
                    }
                }
            }
        }
    }
}