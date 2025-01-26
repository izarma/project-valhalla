use bevy::prelude::*;

#[derive(Component, Default)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
}

pub enum AnimationEventKind {
    Finished,
    _Charging,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub kind: AnimationEventKind,
    pub entity: Entity,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut SpriteAnimState, Entity)>,
    mut event_writer_anim: EventWriter<AnimationEvent>,
)
{
    for (mut sprite, mut anim_state, entity) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                info!("Current Index: {} End Index: {}", atlas.index, anim_state.end_index);
                atlas.index += 1;
                if atlas.index > anim_state.end_index {
                    atlas.index = anim_state.start_index;
                     event_writer_anim.send(AnimationEvent {
                         kind: AnimationEventKind::Finished,
                         entity,
                 });
                }
        }
        }
    }
}