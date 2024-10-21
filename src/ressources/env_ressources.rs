use bevy::prelude::{Resource, Timer};
// use bevy_rand::prelude::ChaCha8Rng;
use rand_chacha::ChaCha8Rng;
use rand_distr::Normal;
use rand::rngs::StdRng;

#[derive(Resource)]
pub struct MoveTimer(pub Timer);

#[derive(Resource)]
pub struct EpisodeTimer(pub Timer);

#[derive(Resource)]
pub struct CumScore(pub f32);

// #[derive(Resource)]
// pub struct RandomDistr(pub Normal<f32>);

#[derive(Resource)]
pub struct RandomGen(pub StdRng);
