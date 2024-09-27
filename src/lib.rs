use bevy::prelude::*;
pub mod components;
pub mod systems;
pub mod ressources;

use ressources::env_ressources::MoveTimer;
use systems::bouncing_ball::*;

// dt of the move timer every 0.05 seconds
const MOVE_DT : f32 = 0.05;

pub struct BounceBall;
impl Plugin for BounceBall {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_env_camera, add_bouncing_ball).chain())
            .insert_resource(MoveTimer(Timer::from_seconds(MOVE_DT, TimerMode::Repeating)))
            .add_systems(Update, ball_dyn_handling);
    }
}
