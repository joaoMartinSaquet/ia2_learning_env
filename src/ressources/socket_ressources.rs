use bevy::prelude::Resource;
use zeromq::PubSocket;


#[derive(Resource)]
pub struct PubSocketRessource(pub PubSocket,);
   