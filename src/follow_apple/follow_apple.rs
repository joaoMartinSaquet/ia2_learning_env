use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::color::palettes::css::WHITE;
use bevy::color::palettes::basic::RED;
use std::io::Write;

use crate::env_common::common::*;
use crate::menu::menu::*;
use crate::score_basics::score::*;
use crate::control::control::*;
use crate::*;


const BALL_RADIUS : f32 = 10.0;
const INIT_VEL_FACTOR : f32 = 3.0;

pub struct FollowApplePlugin;
impl Plugin for FollowApplePlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(DirTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
           .insert_resource(DirDrawed(false))
           .add_systems(FixedUpdate, (run_trajectory).run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::FollowApple)).before(score_metric).before(dumps_log))
           .add_systems(FixedUpdate, change_direction.run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::FollowApple)))
           .add_systems(FixedUpdate, move_player.run_if(in_state(RunningState::Running)).before(dumps_log))
           .add_systems(FixedUpdate, (score_metric, set_log_string).chain().run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::FollowApple)))

           
           .add_systems(OnEnter(TaskState::FollowApple), setup_env_follow_apple)
           .add_systems(OnEnter(RunningState::Started), restart)
        ;
    }
}

fn setup_env_follow_apple(mut commands: bevy::prelude::Commands, 
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

fn restart(mut query_transform : Query<(&mut Transform, &mut Velocity)>,
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


fn move_player(mut query: Query<(&mut Transform, &NameComponent)>,
                   windows: Query<&Window>,
                   last_cmd : Res<LastCmdDisplacement>)
{
    
    let width = windows.single().width();
    let dx = last_cmd.dx;
    let _dy = last_cmd.dy;


    for (mut transform, name) in query.iter_mut()
    {
        if name.0 == "player".to_string()
        {   
            let x_player = transform.translation.x;
            let _y_player = transform.translation.y;

            if f32::abs(x_player + dx) < width/2. {transform.translation.x += dx }
        }
    }
}

fn set_log_string(mut log_str : ResMut<LogStr>,
                query: Query<(&Transform, &NameComponent)>, 
                cum_score : Res<CumScore>, 
                episode_timer : Res<EpisodeTimer>,
                last_cmd_d : Res<LastCmdDisplacement>)
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

    log_str.0 = format!("{:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2};\n", 
        ball_pose_x, ball_pose_y, player_pose_x, player_pose_y, cmd_dx, cmd_dy, score, time);

}