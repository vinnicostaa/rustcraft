use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use rc_player::PlayerControlState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
}

pub struct RustcraftStatePlugin;

impl Plugin for RustcraftStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, toggle_pause)
            .add_systems(OnEnter(GameState::InGame), capture_cursor)
            .add_systems(OnEnter(GameState::Paused), release_cursor);
    }
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_player_control: ResMut<NextState<PlayerControlState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    match state.get() {
        GameState::InGame => {
            next_state.set(GameState::Paused);
            next_player_control.set(PlayerControlState::Disabled);
        }
        GameState::Paused => {
            next_state.set(GameState::InGame);
            next_player_control.set(PlayerControlState::Enabled);
        }
    }
}

fn capture_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn release_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}
