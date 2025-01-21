use bevy::{prelude::*, utils::HashMap};
use crate::multiplayer::config::Config;
use crate::engine::fighter::Fighter;
use bevy_ggrs::*;

const INPUT_LEFT: u8 = 1 << 0;
const INPUT_RIGHT: u8 = 1 << 1;
const INPUT_ATTACK: u8 = 1 << 2;
const INPUT_SPECIAL: u8 = 1 << 3;
const INPUT_GUARD: u8 = 1 << 4;

#[derive(Event, Debug)]
pub enum FighterActions {
    Attack(usize),
    Special(usize),
    Guard(usize)
}


pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;

        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_ATTACK;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_SPECIAL;
        }
        if keys.any_pressed([KeyCode::Enter, KeyCode::KeyE]) {
            input |= INPUT_GUARD;
        }

        local_inputs.insert(*handle, input);
    }
    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

pub fn move_players(
    mut fighters: Query<(&mut Transform, &Fighter)>,
    inputs: Res<PlayerInputs<Config>>,
    mut fighter_action_event: EventWriter<FighterActions>,
    time: Res<Time>,
) {
    for (mut transform, fighter) in &mut fighters {
        let (input, _) = inputs[fighter.handle];

        let mut direction = Vec2::ZERO;

        if input & INPUT_ATTACK != 0 {
            fighter_action_event.send(FighterActions::Attack(fighter.handle));
            info!("Player {} is Attacking", fighter.handle);
        }
        if input & INPUT_SPECIAL != 0 {
            fighter_action_event.send(FighterActions::Special(fighter.handle));
            info!("Player {} is Special Attacking", fighter.handle);
        }
        if input & INPUT_GUARD != 0 {
            fighter_action_event.send(FighterActions::Guard(fighter.handle));
            info!("Player {} is Guarding", fighter.handle);
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_secs();
        transform.translation += move_delta.extend(0.);
    }
}