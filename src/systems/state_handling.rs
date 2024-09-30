use bevy::prelude::EventReader;
use bevy::input::keyboard::{KeyboardInput, KeyCode};
use bevy::prelude::{Res, ResMut, State, NextState};
use bevy::input::ButtonState;
use crate::RunningState;



/// This system prints out all keyboard events as they come in
pub fn toggle_run_pause(mut keyboard_input_events: EventReader<KeyboardInput>,
                    state: Res<State<RunningState>>,
                    mut next_state: ResMut<NextState<RunningState>>) {

    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            if event.key_code == KeyCode::KeyS {
                match state.get() {
                    RunningState::Paused => next_state.set(RunningState::Running),
                    RunningState::Running => next_state.set(RunningState::Paused),
                }
            }
        }
    }
}
