pub mod components;
pub mod systems;
pub mod ressources;

use ressources::env_ressources::MoveTimer;
use systems::{env_systems::*, state_handling::toggle_run_pause};
use bevy::prelude::*;

// dt of the move timer every 0.05 seconds
const MOVE_DT : f32 = 0.005;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunningState {
    #[default]
    Paused,
    Running,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ControllerState {
    #[default]
    Mouse,
    Running,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MyInputKindSet {
    Touch,
    Mouse,
    Gamepad,
}


pub struct BounceBall;
impl Plugin for BounceBall {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)))
            .insert_resource(Time::<Fixed>::from_seconds(0.0001))
            .init_state::<RunningState>()
            .add_systems(Startup,setup_bouncing_ball)
            .insert_resource(MoveTimer(Timer::from_seconds(MOVE_DT, TimerMode::Repeating)))
            .add_systems(FixedUpdate, (ball_dyn_handling).run_if(in_state(RunningState::Running)));
            
    }
}

pub struct  LearningEnv;
impl Plugin for LearningEnv
{
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)))
           .insert_resource(Time::<Fixed>::from_seconds(0.0001))
           .init_state::<RunningState>()
           .init_state::<ControllerState>()
        //    .configure_sets(Update, (ControlSet.run_if))
           .add_systems(Startup, setup_env)
           .add_systems(Update, toggle_run_pause)
           .add_systems(FixedUpdate, (run_trajectory).run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, (mouse_control).run_if(in_state(ControllerState::Mouse)).run_if(in_state(RunningState::Running)));
    }
}

