use bevy::prelude::*;

// default component
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

// #[derive(Component)]
// pub struct Running(pub bool);  

#[derive(Component)]
pub struct Name(pub String);