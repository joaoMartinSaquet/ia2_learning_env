// file containing utils and common systems


use core::f32;
use std::io::Write;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::color::palettes::basic::{RED, BLACK};
use rand::Rng;
use crate::components::env_component::*;
use crate::ressources::env_ressources::*;
use crate::score_basics::score::{gaussian_score, square_score};
use crate::trajectory_basics::trajectory_handling::*;
// use std::fs::File;
use std::fs::*;
use std::io::Read;
use zeromq::{PubSocket, SubSocket};



use crate::menu::menu::*;
use crate::control::control::*;

use crate::UPDT;

const BALL_RADIUS : f32 = 10.0;
const ELASTIC_COEF : f32 = 0.7;
const ACCEL_TIME : f32 = 5.0;
// const T : f32 = 30.;
const DIR_CHGT : f32 = 1.0;
const INIT_VEL_FACTOR : f32 = 3.0;
const GSCORE : bool = false;
const INPUT_FILE : &str = "input/smooth_input.in";


#[allow(dead_code)]
enum Trajectory {
    Linear,
    Random,
    Cos,
    NonMoving,
}

const TRAJECTORY_TO_RUN : Trajectory = Trajectory::Cos;

// Ressources
#[derive(Resource)]
pub struct FileInput(pub Vec<String>);

#[derive(Resource)]
pub struct PubSocketRessource(pub PubSocket,);

#[derive(Resource)]
pub struct SubSocketRessource(pub SubSocket,);
   

pub fn move_ball(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,)
{

    for (mut transform, ball_velocity) in query.iter_mut() {
        // println!("translate {:?}", transform);
        transform.translation.x += ball_velocity.dx * time.delta_seconds();
        transform.translation.y += ball_velocity.dy * time.delta_seconds();
    }

}

/// Handle the dynamic of the ball.
///
/// In this function, we manage the dynamic of the ball. The ball's position is updated
/// according to its velocity, and its velocity is updated according to the gravity.
/// When the ball hit the ground, its velocity is reversed and reduced by the coefficient
/// of restitution.
///
/// The function takes a mutable reference to a `Query` containing the ball's `Transform`
/// and `Velocity` components, and a `Res` containing the `Time` resource.
///
/// The function returns nothing.
pub fn ball_dyn_handling(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {

    // println!("Running state: {:?}", state.get());
    // println!("Elapsed seconds: {}", time.delta_seconds());
    let dt = time.delta_seconds()*ACCEL_TIME;
    // println!("delta time {:?}", dt);
     
    for (mut transform, mut ball_velocity) in query.iter_mut() {

        let _x = transform.translation.x;
        let y = transform.translation.y;
        // println!("position of the ball {:?} {:?}", _x, y);
        if y <= BALL_RADIUS && ball_velocity.dy <= 0.0 {
            // println!("Ball hit on the ground with velovity {:?}", ball_velocity.dy);
            ball_velocity.dy = -ELASTIC_COEF*ball_velocity.dy;
                
        }else {
            ball_velocity.dy = ball_velocity.dy - 9.81*dt;
        }
        transform.translation.y = transform.translation.y + ball_velocity.dy * dt;        

    }
}

pub fn command_desc_text(commands: &mut bevy::prelude::Commands, asset_server: Res<AssetServer>){
    
    commands.spawn(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "To run / pause the sim, press {s}",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 25.0,
                color : Color::WHITE,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }));
}

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

/// Function used to run the ball trajectory.
///
/// # Arguments
///
/// * `query` - a query on the entities with a `Transform`, `Velocity` and `NameComponent` components
/// * `time` - the time ressource
/// * `episode_timer` - the episode timer ressource
/// * `windows` - a query on the windows
/// * `dir_drawed` - a mutable ressource used to store the direction of the last drawn ball
///
/// # Description
///
/// The function will run the trajectory of the ball based on the current time of the episode.
/// The trajectory can be linear, random, cosinus or non moving.
/// For the random trajectory, the direction of the ball will change every `DIR_CHGT` seconds.
/// For the cosinus trajectory, the ball will move in a cosinus function.
/// For the linear trajectory, the ball will move with a constant velocity.
/// For the non moving trajectory, the ball will not move.
pub fn run_trajectory(mut query: Query<(&mut Transform, &mut Velocity, &NameComponent)>,
                          time: Res<Time>,
                          episode_timer : Res<EpisodeTimer>,
                          windows: Query<&Window>,
                          mut dir_drawed : ResMut<DirDrawed>,)
{
    // time elapsed
    let window = windows.single();
    let width = window.width();
    // let dir_drawed : f32 = (rng.gen_bool(0.5) as i32 * 2 - 1) as f32;
    // println!("time : {:?} direction drawed {:?}", episode_timer.0.elapsed().as_secs_f32(), dir_drawed);
    // println!("run traj on time : {:?} ", episode_timer.0.elapsed().as_secs_f32());
    for (mut transform,mut vel, name) in query.iter_mut()
    {
        if name.0 == "follow object".to_string()
        {
            let mut _dx = 0.0;
            let dt = time.delta_seconds();
            match TRAJECTORY_TO_RUN {
                Trajectory::Linear => {
                                        _dx = linear_dx_trajectory(transform.translation.x, dt, &mut vel.dx, width);
                                      },
                Trajectory::Random => {
                                        if episode_timer.0.elapsed().as_secs_f32() % DIR_CHGT < UPDT as f32 {
                                            vel.dx = ((dir_drawed.0 as i32)*2 -1) as f32 * vel.dx;
                                        }
                                        
                                        if f32::abs(transform.translation.x + vel.dx * dt) > width/2.0 {
                                            vel.dx = -vel.dx;
                                            dir_drawed.0 = !dir_drawed.0 ;
                                        }
                                        _dx = vel.dx * dt;
                                      },
                Trajectory::Cos => {
                                        _dx = cosinus_dx_trajectory(episode_timer.0.elapsed_secs(), dt, width);
                }
                Trajectory::NonMoving => {
                        _dx  = 0.0;
                }
            }
            transform.translation.x += _dx;
        }
    }
}

/// Sets up the bouncing ball environment by spawning the ball and the ground.
/// 
/// This function spawns a bouncing ball entity with an initial position and velocity,
/// as well as a ground entity to provide a surface for the ball to bounce on.
/// 
/// # Parameters
/// 
/// - `commands`: A mutable reference to the Bevy `Commands` object, used for spawning entities.
/// - `asset_server`: A reference to the Bevy `AssetServer`, used to load assets.
/// - `meshes`: A mutable reference to the Bevy `Assets<Mesh>`, used to add meshes.
/// - `materials`: A mutable reference to the Bevy `Assets<ColorMaterial>`, used to add materials.
/// 
/// The ball is represented as a circle with a defined radius and a red color. The ground is represented
/// as a rectangle with a black color.
pub fn setup_bouncing_ball(mut commands: bevy::prelude::Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,)
{
    let y_obj = 200.0;
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(BALL_RADIUS)).into(),
        material: materials.add(Color::from(RED)),
        transform: Transform::from_xyz( 0.0,
            y_obj,
            0.0,
        ),
        ..default()
        }, 
        Velocity {dx: 0.0, dy: 0.0},         NameComponent("bouncing_ball".to_string() ) )

    );
    

    // spawn the ground
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(1000., 5.)).into(),
        material: materials.add(Color::from(BLACK)),
        transform: Transform::from_xyz(
            0.0,
            -5.0,
            0.0,
        ),
        ..default()
    });

    command_desc_text(&mut commands, asset_server);
}

/// Calculate the score of the player based on its position and the position of the follow object
/// 
/// The score is calculated as a function of the distance between the player and the follow object.
/// The function used is determined by the variable `GSCORE`.
/// If `GSCORE` is true, the score is calculated using a Gaussian function.
/// If `GSCORE` is false, the score is calculated using a square function.
/// The score is then added to the cumulative score and displayed on screen.
pub fn score_metric(query: Query<(&Transform, &NameComponent)>,
                    mut query_text: Query<&mut Text, With<ScoreTxt>>,
                    mut cumscore : ResMut<CumScore>,)
{

    let mut x_player = 0.0;
    let mut x_folow = 0.0;
    // println!("score_metrics on time : {:?} ", time.0.elapsed().as_secs_f32());
    for (transform, name) in query.iter()
    {
        if name.0 == "follow object".to_string()
        {
            x_folow = transform.translation.x;
        }
        if name.0 == "player".to_string()
        {
            x_player = transform.translation.x;
        }        
    }   

    let score ;
    // + eps to avoid division by zero
    // let score = 1./(f32::abs(x_folow - x_player) + 0.01);
    if GSCORE 
    {
        score = gaussian_score(x_player, x_folow);
    }else {
        score = square_score(x_player, x_folow);
    }
    //

    // println!("score {:?} ", score);
    
    cumscore.0 += score;
    let disp_score = cumscore.0;
    
    for mut text in query_text.iter_mut()
    {
        text.sections[1].value = format!("{disp_score:.2}");
        // println!("score {:?}", score);
    }
    
}

pub fn displays_cum_score(cum_score : Res<CumScore>,)
{
    println!("total score is : {:?}", cum_score.0)

}

pub fn restart(mut query_transform : Query<(&mut Transform, &mut Velocity)>,
               mut cumscore : ResMut<CumScore>, 
               mut episode_timer : ResMut<EpisodeTimer>,)
{
    
    for (mut transform, mut vel) in query_transform.iter_mut()
    {
        transform.translation.x = 0.0;
        // 1-D axis transform.translation.y = 0.0;
        vel.dx = f32::abs(vel.dx);
    }

    // reset ressources
    cumscore.0 = 0.0;
    episode_timer.0.reset();
}

pub fn dumps_log(query: Query<(&Transform, &NameComponent)>, 
                 cum_score : Res<CumScore>, 
                 episode_timer : Res<EpisodeTimer>,
                 last_cmd_d : Res<LastCmdDisplacement>,
                 mut data_file : ResMut<LogFile>)
{

    let mut player_pose_x = 0.0;
    let mut player_pose_y = 0.0;
    let mut ball_pose_x = 0.0;
    let mut ball_pose_y = 0.0;
    let cmd_dx = last_cmd_d.dx;
    let cmd_dy = last_cmd_d.dy;
    let score = cum_score.0;
    let time = episode_timer.0.elapsed().as_secs_f32();
    // println!(" dumps log on time : {:?} ", time);
    // get the player and ball pose 
    for (transform, name) in query.iter()
    {
        if name.0 == "player".to_string()
        {   
            player_pose_x = transform.translation.x;
            player_pose_y = transform.translation.y;
        }
        if name.0 == "follow object".to_string()
        {   
            ball_pose_x = transform.translation.x;
            ball_pose_y = transform.translation.y;
        }
    }

    if !episode_timer.0.finished()
    {
        let log_str = format!("{:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2};\n", 
        ball_pose_x, ball_pose_y, player_pose_x, player_pose_y, cmd_dx, cmd_dy, score, time);
        data_file.0.write(log_str.as_bytes()).expect("write failed");
    }


}

pub fn change_direction(mut dir : ResMut<DirDrawed>, 
                        mut random_source: ResMut<RandomGen>,
                        mut timer : ResMut<DirTimer>,
                        time: Res<Time>               
                        )
{
    // this function draw a new direction for the ball to follow 
    // we want to draw a new direction every DIR_CHGT
    if timer.0.tick(time.delta()).just_finished()
    {

        dir.0 = random_source.0.gen_bool(0.5);
        // println!(" draw new direction : time  {:?} with dir drawed  : {:?}", timer.0.elapsed_secs(), dir.0);
    }
    
}

pub fn despawn_objects(mut commands: Commands, query: Query<Entity, With<NameComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_cam(mut commands: Commands, asset_server: Res<AssetServer>,) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        transform: Transform{ translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, scale : Vec3 { x: 0.3, y: 0.3, z: 1.0 }, ..default()},
        texture : asset_server.load("./background/background.png"),
        ..default()}, 
        ));
}

pub fn read_input_from_file(mut file_input : ResMut<FileInput>)
{
    let mut file: File = File::open(INPUT_FILE).unwrap();
    let mut content : String = String::new();

    file.read_to_string(&mut content).unwrap();
    let mut v : Vec<String> = content.split("\n").map(|x| x.to_string()).collect();
    v.remove(v.len()-1);
    v.remove(0);
    file_input.0 = v;
}
