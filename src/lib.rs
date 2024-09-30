use bevy::prelude::*;
pub mod components;
pub mod systems;
pub mod ressources;

use ressources::env_ressources::MoveTimer;
use systems::{env_systems::*, state_handling::toggle_run_pause};

// dt of the move timer every 0.05 seconds
const MOVE_DT : f32 = 0.005;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunningState {
    #[default]
    Paused,
    Running,
}


pub struct BounceBall;
impl Plugin for BounceBall {
    fn build(&self, app: &mut App) {
        app
            .init_state::<RunningState>()
            .add_systems(Startup, (spawn_env_camera, command_desc_text ,add_bouncing_ball).chain())
            .insert_resource(MoveTimer(Timer::from_seconds(MOVE_DT, TimerMode::Repeating)))
            .add_systems(FixedUpdate, (ball_dyn_handling).run_if(in_state(RunningState::Running)))
            .add_systems(Update, toggle_run_pause);
            
    }
}
