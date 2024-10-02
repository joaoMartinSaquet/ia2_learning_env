use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct MoveTimer(pub Timer);

#[derive(Resource)]
pub struct EpisodeTimer(pub Timer);
