use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub frame_size: UVec2,
    pub timer: Timer,
}

pub enum AnimationEventKind {
    Finished,
    _Charging,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub kind: AnimationEventKind,
    pub _entity: Entity,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut SpriteAnimState)>,
    //mut event_writer_anim: EventWriter<AnimationEvent>,
)
{
    for (mut sprite, mut anim_state) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index += 1;
                info!("{}",atlas.index);
                if atlas.index > anim_state.end_index {
                    atlas.index = anim_state.start_index;
                //     event_writer_anim.send(AnimationEvent {
                //         kind: AnimationEventKind::Finished,
                //         _entity,
                // });
                }
        }
        }
    }
}