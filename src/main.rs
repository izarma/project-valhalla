use bevy::{prelude::*, render::camera::ScalingMode};

mod engine;
mod multiplayer;
mod animations;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    Matchmaking,
    Lobby,
    InGame,
    GameOver,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum RollbackState {
    /// When the characters fighting
    #[default]
    InRound,
    /// When one character is dead, and we're transitioning to the next round
    RoundEnd,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // fill the entire browser window
            fit_canvas_to_parent: true,
            // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .init_state::<GameState>()
    .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
    .add_systems(Startup, setup)
    .add_plugins(engine::game_runner::GameRunnerPlugin)
    .run();
}


fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}





