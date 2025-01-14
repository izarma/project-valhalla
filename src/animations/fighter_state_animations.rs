use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct PlayerState(pub Vec<PlayerStateKind>);

impl PlayerState {
    pub fn current_state(&self) -> PlayerStateKind {
        *self.0.last().unwrap_or(&PlayerStateKind::Idle)
    }
    pub fn push_state(&mut self, new_state: PlayerStateKind) {
        // Add new state
        self.0.push(new_state);
    }
}

#[derive(Event, Debug, Copy, Clone)]
pub enum PlayerStateKind {
    Idle,
    Walk,
    Dash,
    Attack,
    AttackHold,
    Special,
    SpecialHold,
    Guard,
}