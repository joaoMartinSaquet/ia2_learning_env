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
pub struct NameComponent(pub String);

#[derive(Component)]
pub struct ScoreTxt;

#[derive(Component)]
pub struct TimeTracker(pub f32);