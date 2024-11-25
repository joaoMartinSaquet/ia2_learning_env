use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::color::palettes::css::WHITE;
use bevy::color::palettes::basic::RED;


const BALL_RADIUS : f32 = 10.0;
const INIT_VEL_FACTOR : f32 = 3.0;

use crate::env_common::common::*;
use crate::menu::menu::*;
use crate::score_basics::score::*;

pub fn setup_env_follow_apple(mut commands: bevy::prelude::Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>)
{
let mut window = windows.single_mut();

let width = window.width();
let _height = window.height();
let y_obj = 200.0;

window.cursor.visible = false;
// let prim_window = windows.single_mut();

// spawn the object to follow, it s a ball
commands.spawn((MaterialMesh2dBundle {
mesh: meshes.add(Circle::new(BALL_RADIUS)).into(),
material: materials.add(Color::from(RED)),
transform: Transform::from_xyz( 0.,
   y_obj,
   1.0,
),
..default()
}, 
Velocity {dx: width/ INIT_VEL_FACTOR, dy: 0.0},         
NameComponent("follow object".to_string() ), 
OnGameScreen )

);

// spawn the players
commands.spawn((SpriteBundle {
                       transform: Transform{ translation: Vec3 { x: 0.0, y: -y_obj, z: 1.0 }, scale : Vec3 { x: 0.3, y: 0.3, z: 1.0 }, ..default()},
                       texture : asset_server.load("./player/player.png"),
                       ..default()}, 
                   NameComponent("player".to_string()), 
                   Velocity {dx: 0.0, dy: 0.0}, 
                   OnGameScreen )
                   );
                   

// spawn score text 
commands.spawn((
// Create a TextBundle that has a Text with a list of sections.
TextBundle::from_sections([
   TextSection::new(
       "Score: ",
       TextStyle {
           // This font is loaded and will be used instead of the default font.
           font: asset_server.load("fonts/FiraSans-Medium.ttf"),
           font_size: 50.0,
           color: Color::from(WHITE),
           ..default()
       },
   ),
   TextSection::from_style(
       // "default_font" feature is unavailable, load a font to use instead.
       TextStyle {
           font: asset_server.load("fonts/FiraSans-Medium.ttf"),
           font_size: 60.0,
           color: Color::from(WHITE),
           ..default()})

]),
ScoreTxt,
OnGameScreen),
);

command_desc_text(&mut commands, asset_server);

}
