use bevy::prelude::Resource;

#[derive(Resource)]
pub struct FileInput(pub Vec<String>);

