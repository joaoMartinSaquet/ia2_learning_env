use core::f32;
use std::io::Write;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::color::palettes::basic::{RED, BLACK};
use crate::components::env_component::*;
use crate::ressources::env_ressources::*;
use crate::score_basics::score::{self, gaussian_score, square_score};
use bevy::input::mouse::MouseMotion;
use crate::trajectory_basics::trajectory_handling::*;


const BALL_RADIUS : f32 = 10.0;
const ELASTIC_COEF : f32 = 0.7;
const ACCEL_TIME : f32 = 5.0;
const T : f32 = 10.;
const DIR_CHGT : f32 = 0.5;
const INIT_VEL_FACTOR : f32 = 3.0;

#[allow(dead_code)]
enum Trajectory {
    Linear,
    Random,
    NonMoving,
}

const TRAJECTORY_TO_RUN : Trajectory = Trajectory::Random;


pub fn spawn_env_camera(commands: &mut bevy::prelude::Commands)
{
    commands
    .spawn(Camera2dBundle::default());// .insert(IA2learningEnvPlugin);
}

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

pub fn setup_env(mut commands: bevy::prelude::Commands, 
             asset_server: Res<AssetServer>,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<ColorMaterial>>,
             windows: Query<&Window>)
{
    let window = windows.single();
    
    let width = window.width();
    let _height = window.height();
    let y_obj = 200.0;

    // spawn the background first to not overwrite 
    commands.spawn(SpriteBundle {
        transform: Transform{ translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 }, scale : Vec3 { x: 0.3, y: 0.3, z: 1.0 }, ..default()},
        texture : asset_server.load("./background/background.png"),
        ..default()});


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
        NameComponent("follow object".to_string() ) )

    );

    // spawn the players
    commands.spawn((SpriteBundle {
                                transform: Transform{ translation: Vec3 { x: 0.0, y: -y_obj, z: 1.0 }, scale : Vec3 { x: 0.3, y: 0.3, z: 1.0 }, ..default()},
                                texture : asset_server.load("./player/player.png"),
                                ..default()}, 
                            NameComponent("player".to_string()), 
                            Velocity {dx: 0.0, dy: 0.0} ));

    spawn_env_camera(&mut commands);

 
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
    ));

    command_desc_text(&mut commands, asset_server);
    
}

pub fn run_trajectory(mut query: Query<(&mut Transform, &mut Velocity, &NameComponent)>,
                          time: Res<Time>,
                          windows: Query<&Window>,
                          mut random_source: ResMut<RandomGen>)
{
    // time elapsed
    let window = windows.single();
    let width = window.width();
    // println!("width {:?} ", window.size());
    let _rad_pulse = 2.0*f32::consts::PI* (1./T);
    let rng = &mut random_source.0;

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
                                        if time.elapsed().as_secs_f32() % DIR_CHGT == 0.0 {
                                            _dx = random_dir_trajectory(vel.dx, rng);
                                            vel.dx = _dx;
                                        }
                                        if f32::abs(transform.translation.x + vel.dx * dt) >= width/2.0 {vel.dx = -vel.dx;}
                                        _dx = vel.dx * dt;
                                      },
                Trajectory::NonMoving => {
                        _dx  = 0.0;
                }
            }
            transform.translation.x += _dx;
        }
    }
}

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

    spawn_env_camera(&mut commands);

    command_desc_text(&mut commands, asset_server);
}

// // fn dx_trajectory(t:f32, dt:f32, rad_pulse:f32, width:f32) -> f32
// // {
//     rad_pulse*(width/2.0)*f32::cos(rad_pulse*(t - 0.0*T/2.0))*dt
// }

/// This system prints out all keyboard events as they come in
pub fn mouse_control(mut mouse_motion: EventReader<MouseMotion>,
                     mut query: Query<(&mut Transform, &NameComponent)>,
                     windows: Query<&Window>,
                     mut last_mouse_movement : ResMut<LastMouseDisplacement>)
{   
    let width = windows.single().width();
    let mut dx = 0.0; 
    let mut dy = 0.0;
    for (mut transform, name) in query.iter_mut()
    {
        if name.0 == "player".to_string()
        {   
            for ev in mouse_motion.read() {

                dx = ev.delta.x;
                dy = ev.delta.y;
                // don't move the player if it's out of bounds
                if f32::abs(transform.translation.x + dx) <= width/2.0
                {
                    transform.translation.x += dx;
                }
                
                // transform.translation.y += ev.delta.y;
            }
        }
    }
    last_mouse_movement.dx = dx;
    last_mouse_movement.dy = dy;    
    // write_to_file_for_now(&mut query); TODO
}

pub fn score_metric(query: Query<(&Transform, &NameComponent)>,
                    mut query_text: Query<&mut Text, With<ScoreTxt>>,
                    mut cumscore : ResMut<CumScore>)
{

    let mut x_player = 0.0;
    let mut x_folow = 0.0;

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

    // + eps to avoid division by zero
    // let score = 1./(f32::abs(x_folow - x_player) + 0.01);

    // let score = gaussian_score(x_player, x_folow);
    let score = square_score(x_player, x_folow);
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
                 mouse_d : Res<LastMouseDisplacement>,
                 mut data_file : ResMut<LogFile>)
{

    let mut player_pose_x = 0.0;
    let mut player_pose_y = 0.0;
    let mut ball_pose_x = 0.0;
    let mut ball_pose_y = 0.0;
    let mouse_dx = mouse_d.dx;
    let mouse_dy = mouse_d.dy;
    let score = cum_score.0;
    let time = episode_timer.0.elapsed().as_secs_f32();

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

    let log_str = format!("{:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2};\n", 
    ball_pose_x, ball_pose_y, player_pose_x, player_pose_y, mouse_dx, mouse_dy, score, time);
    data_file.0.write(log_str.as_bytes()).expect("write failed");

}