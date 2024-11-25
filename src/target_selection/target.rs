use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::menu::menu::OnGameScreen;
// use bevy::prelude::Color::rgb;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Target;


const TARGET_RADIUS_INIT : f32 = 100.0;
// const target_color : Color = Color::Srgba(0.636, 0.968, 0.596);
const TARGET_COLOR : Color=  Color::srgba(0.636, 0.968, 0.596, 0.3);

pub fn setup_target(mut commands: bevy::prelude::Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>)
{

    let window  = windows.single_mut();
    let width = window.width();
    let height = window.height();

    let target_x : f32 = width/3.0;
    let target_y : f32 = height/3.0;

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(TARGET_RADIUS_INIT)).into(),
        material: materials.add(TARGET_COLOR),
        transform: Transform::from_xyz( target_x,
            target_y,
            1.0,
        ),
        ..default()
        },           
        Target,
        OnGameScreen )
    );


    commands.spawn((SpriteBundle {
        transform: Transform{ translation: Vec3 { x: 0.0, y: 0.0, z: 1.0 }, scale : Vec3 { x: 0.1, y: 0.1, z: 1.0 }, ..default()},
        texture : asset_server.load("./player/knife.png"),
        ..default()},  
        Player,
        OnGameScreen )
    );


}