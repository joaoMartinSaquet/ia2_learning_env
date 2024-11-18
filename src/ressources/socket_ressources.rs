use bevy::prelude::Resource;
use zeromq::{PubSocket, SubSocket};


#[derive(Resource)]
pub struct PubSocketRessource(pub PubSocket,);

#[derive(Resource)]
pub struct SubSocketRessource(pub SubSocket,);
   