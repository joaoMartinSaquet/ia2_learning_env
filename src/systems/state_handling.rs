use bevy::prelude::EventReader;
use bevy::input::keyboard::{KeyboardInput, KeyCode};
use bevy::prelude::{Res, ResMut, State, NextState, Time};
use bevy::input::ButtonState;
use crate::ressources::env_ressources::EpisodeTimer;
use crate::{ControllerState, RunningState, NetworkState};



/// This system prints out all keyboard events as they come in
pub fn toggle_run_pause(mut keyboard_input_events: EventReader<KeyboardInput>,
                    state: Res<State<RunningState>>,
                    mut next_state: ResMut<NextState<RunningState>>) {

 
    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            // println!("state {:?}", state.get());
            // println!("{:?}", event);
            if event.key_code == KeyCode::KeyS {
                match state.get() {
                    RunningState::Running => next_state.set(RunningState::Paused),
                    RunningState::Paused => next_state.set(RunningState::Running),  
                    RunningState::Started => next_state.set(RunningState::Running),        
                    _ => ()
                }
            }
            // println!("restart condition {:?} ",event.key_code == KeyCode::KeyR && *state.get() == RunningState::Ended);
            if event.key_code == KeyCode::KeyR && *state.get() == RunningState::Ended {
                println!("changing state ");
                next_state.set(RunningState::Started);
            }
            else if event.key_code == KeyCode::KeyE && *state.get() == RunningState::Paused {
                next_state.set(RunningState::Ended);
            }
        }
    }
}

pub fn run_episodes_timer(state: Res<State<RunningState>>,
                     mut next_state: ResMut<NextState<RunningState>>,
                     time: Res<Time>,
                     mut episode_timer : ResMut<EpisodeTimer>)
{
    // this function is decreasing the timer when the game is running and handle the time change

    if *state.get() == RunningState::Running { 
        // println!("episode timer {:?}", episode_timer.0);
        episode_timer.0.tick(time.delta());
    }

    if episode_timer.0.just_finished() && *state.get() == RunningState::Running {
        next_state.set(RunningState::Ended);
    }
}

pub fn controller_choice(mut keyboard_input_events: EventReader<KeyboardInput>,
                        state: Res<State<ControllerState>>,
                        mut next_state: ResMut<NextState<ControllerState>>) {
    
    
    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            // println!("Changing controller state  {:?} event keycode : {:?}", state.get(), event.key_code);
            match state.get() {
                ControllerState::Mouse => match event.key_code {
                                            KeyCode::KeyF => next_state.set(ControllerState::InputFile),
                                            KeyCode::KeyA => next_state.set(ControllerState::Sub),
                                            _ => next_state.set(ControllerState::Mouse),}
                
                
                ControllerState::InputFile => match event.key_code {
                                                KeyCode::KeyM => next_state.set(ControllerState::Mouse),
                                                KeyCode::KeyA => next_state.set(ControllerState::Sub),
                                                _ => next_state.set(ControllerState::InputFile),}
                
                ControllerState::Sub => match event.key_code {
                                        KeyCode::KeyM => next_state.set(ControllerState::Mouse),
                                        KeyCode::KeyF => next_state.set(ControllerState::InputFile),
                                        _ => next_state.set(ControllerState::Sub),}
                    
                }
                // _ => next_state.set(ControllerState::Mouse)
            }
        }

        
    }

pub fn networking_choice(mut keyboard_input_events: EventReader<KeyboardInput>,
                        state: Res<State<NetworkState>>,
                        mut next_state: ResMut<NextState<NetworkState>>) {
    

    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            
            match state.get() {
                NetworkState::Unconnected => if event.key_code == KeyCode::KeyN {next_state.set(NetworkState::Connected);}
                NetworkState::Connected => if event.key_code == KeyCode::KeyN  {next_state.set(NetworkState::Unconnected);}
            }
            // println!("Changing network state #{:?} ----> #{:?} || event keycode : {:?}", state.get(), next_state, event.key_code);
        }
    }
}