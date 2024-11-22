use std::fs::File;

use bevy::prelude::{Resource, Timer};
// use bevy_rand::prelude::ChaCha8Rng;
// use rand::rngs::StdRng;
use rand_chacha::ChaCha8Rng;
#[derive(Resource)]
pub struct MoveTimer(pub Timer);

#[derive(Resource)]
pub struct EpisodeTimer(pub Timer);

#[derive(Resource)]
pub struct CumScore(pub f32);

// #[derive(Resource)]
// pub struct RandomDistr(pub Normal<f32>);

#[derive(Resource)]
pub struct RandomGen(pub ChaCha8Rng);

// last displacement given by a controller (mouse, file, networks etc...)
#[derive(Resource)]
pub struct LastCmdDisplacement
{
    pub dx: f32,
    pub dy: f32
}

#[derive(Resource)]
pub struct LogFile(pub File);

# [derive(Resource)]
pub struct DirDrawed(pub bool);

#[derive(Resource)]
pub struct DirTimer(pub Timer);
