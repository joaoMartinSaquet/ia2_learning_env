// file containing utils and common systems
use core::f32;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::prelude::*;
use bevy::color::palettes::basic::{RED, BLACK};

use std::fs::*;
use std::io::Read;
use zeromq::{PubSocket, SubSocket, Socket, SocketSend, ZmqMessage};
use rand_chacha::ChaCha8Rng;
use core::str;
use bevy::prelude::{Query, Res, ResMut, Transform};
use bevy::input::keyboard::*;
use bevy::input::ButtonState;
use crate::*;



const SERVER : &str = "tcp://127.0.0.1";   
const LOG_PORT : &str = "5556";
const CMD_PORT : &str = "5560";

const LOG_TOPIC : &str = "GameData/";
const CMD_TOPIC : &str = "";


const BALL_RADIUS : f32 = 10.0;
const ELASTIC_COEF : f32 = 0.7;
const ACCEL_TIME : f32 = 5.0;

const INPUT_FILE : &str = "input/smooth_input.in";


// Ressources
#[derive(Resource)]
pub struct FileInput(pub Vec<String>);

#[derive(Resource)]
pub struct PubSocketRessource(pub PubSocket,);

#[derive(Resource)]
pub struct SubSocketRessource(pub SubSocket,);

#[derive(Resource)]
pub struct EpisodeTimer(pub Timer);

#[derive(Resource)]
pub struct RandomGen(pub ChaCha8Rng);

#[derive(Resource)]
pub struct LogFile(pub File);

#[derive(Resource)]
pub struct LogStr(pub String);

// Components
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

#[derive(Component)]
pub struct NameComponent(pub String);


#[derive(Component)]
pub struct TimeTracker(pub f32);



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

pub fn toggle_run_pause(mut keyboard_input_events: EventReader<KeyboardInput>,
    state: Res<State<RunningState>>,
    mut next_state: ResMut<NextState<RunningState>>) {


    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            // println!("state {:?}", state.get());
            // println!("{:?}", event);
            if event.key_code == KeyCode::KeyS {
                match state.get() {
                    RunningState::Running => next_state.set(RunningState::Paused),
                    RunningState::Paused => next_state.set(RunningState::Running),  
                    RunningState::Started => next_state.set(RunningState::Running),        
                    _ => ()
                }
            }
            // println!("restart condition {:?} ",event.key_code == KeyCode::KeyR && *state.get() == RunningState::Ended);
            if event.key_code == KeyCode::KeyR && *state.get() == RunningState::Ended {
                println!("changing state ");
                next_state.set(RunningState::Started);
            }
            else if event.key_code == KeyCode::KeyE && *state.get() == RunningState::Paused {
                next_state.set(RunningState::Ended);
            }
        }
    }
}

pub fn run_episodes_timer(state: Res<State<RunningState>>,
    mut next_state: ResMut<NextState<RunningState>>,
    time: Res<Time>,
    mut episode_timer : ResMut<EpisodeTimer>)
{
    // this function is decreasing the timer when the game is running and handle the time change

    if *state.get() == RunningState::Running { 
        // println!("episode timer {:?}", episode_timer.0);
        episode_timer.0.tick(time.delta());
    }

    if episode_timer.0.just_finished() && *state.get() == RunningState::Running {
        next_state.set(RunningState::Ended);
    }
}

pub fn networking_choice(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    state: Res<State<NetworkState>>,
    mut next_state: ResMut<NextState<NetworkState>>,) {
    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            match state.get() {
                NetworkState::Unconnected => {
                    if event.key_code == KeyCode::KeyN {
                        next_state.set(NetworkState::Connected);
                    }
                }
                NetworkState::Connected => {
                    if event.key_code == KeyCode::KeyN {
                        next_state.set(NetworkState::Unconnected);
                    }
                }
            }
            // println!("Changing network state #{:?} ----> #{:?} || event keycode : {:?}", state.get(), next_state, event.key_code);
        }
    }
}

#[tokio::main]
pub async  fn initialize_pub_sub_connection(mut pub_socket : ResMut<PubSocketRessource>,
                                            mut sub_socket : ResMut<SubSocketRessource>)
{   
    let pub_server = SERVER.to_owned() + ":" + LOG_PORT;
    let sub_server = SERVER.to_owned() + ":" + CMD_PORT;
    // ingoring error because System doesn't handle error 
    let _ = pub_socket.0.bind(&pub_server).await;
    println!("publisher Socket binded to {}", pub_server);

    let _ = sub_socket.0.bind(&sub_server).await;
    let _ = sub_socket.0.subscribe(CMD_TOPIC).await;
    println!("sublisher Socket connected to {}", sub_server);

}

#[tokio::main]
pub async fn publish_log(log_str : Res<LogStr>,
                         mut pub_socket : ResMut<PubSocketRessource>)
{


    let mut m: ZmqMessage = ZmqMessage::from(LOG_TOPIC);
    m.push_back(log_str.0.as_bytes().to_vec().into());

    let e = pub_socket.0.send(m).await;

    if e .is_err()
    {
        println!("Error while sending message");
    }
    
}

pub fn dumps_log(log_str : Res<LogStr>,
                 mut data_file : ResMut<LogFile>, 
                 episode_timer : Res<EpisodeTimer>)
{

    if !episode_timer.0.finished()
    {
        
        data_file.0.write(log_str.0.as_bytes()).expect("write failed");
    }

}


