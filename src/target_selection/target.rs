use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::menu::menu::OnGameScreen;
use crate::control::control::LastCmdDisplacement;
use crate::{CumScore, RunningState, TaskState};
use crate::env_common::common::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Target;


const TARGET_RADIUS_INIT : f32 = 100.0;
const TARGET_COLOR : Color=  Color::srgba(0.636, 0.968, 0.596, 0.3);
const TARGET_MIN_RADIUS :f32 =  20.;

// time the player stay inside the target
const TIMER_TARGET_STAY : f32 = 1.0;


const TARGET_X : [f32; 6]= [0.0, 300., -300. , 0.0, 300., -300.];
const TARGET_Y : [f32; 6]= [0.0, 300., -300., 300., -300., 300.];

#[derive(Resource)]
pub struct TargetRadius(f32);

#[derive(Resource)]
pub struct TimerTarget(Timer);


#[derive(Resource)]
pub struct TargetNum(i32);

pub struct TargetSelectionPlugin;
impl Plugin for TargetSelectionPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(TargetRadius(TARGET_RADIUS_INIT))
           .insert_resource(TimerTarget(Timer::from_seconds(TIMER_TARGET_STAY, TimerMode::Repeating)))
           .insert_resource(TargetNum(0))
           
           .add_systems(OnEnter(TaskState::TargetSelection), setup_target)
           .add_systems(Update, (move_entity).run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::TargetSelection)))
           .add_systems(FixedUpdate, (is_player_in_target, set_log_string).run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::TargetSelection)));
    }
}



pub fn setup_target(mut commands: bevy::prelude::Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
    mut target_num : ResMut<TargetNum>)
{

    let window  = windows.single_mut();
    let width = window.width();
    let height = window.height();

    target_num.0 = 0;

    let target_x : f32 = TARGET_X[target_num.0 as usize];
    let target_y : f32 = TARGET_Y[target_num.0 as usize];

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
        transform: Transform{ translation: Vec3 { x: 0.0, y: 0.0, z: 1.0 }, scale : Vec3 { x: 0.05, y: 0.05, z: 1.0 }, ..default()},
        texture : asset_server.load("./player/knife.png"),
        ..default()},  
        Player,
        OnGameScreen )
    );


}

fn move_entity(mut query: Query<&mut Transform, (With<Player>, Without<Target>)>,
               cmd :Res<LastCmdDisplacement>)
{

    for mut player_transformation in query.iter_mut()
    {
        // println!("command is dx : {:?} dy : {:?}", cmd.dx, cmd.dy);
        player_transformation.translation.x += cmd.dx;
        player_transformation.translation.y += cmd.dy;
    }

}

/// This function checks if the player is in the target zone.
/// 
/// The function takes as input a query on the player's position and a query on the target's position.
/// It also takes a mutable reference to a `TimerTarget` which is used to keep track of the time the player has been inside the target.
/// The function also takes a mutable reference to a `TargetRadius` which is used to keep track of the current radius of the target.
/// 
/// The function first gets the position of the player and the target from the queries.
/// It then calculates the distance between the player and the target.
/// If the distance is less than or equal to the current radius of the target, the function ticks the timer and checks if the timer has just finished.
/// If the timer has just finished, the function divides the current radius of the target by 2 and sets the scale of the target to the new radius divided by the initial radius.
/// The function then resets the timer.
/// If the distance is greater than the current radius of the target, the function resets the timer.
pub fn is_player_in_target(query_player: Query<&Transform, With<Player>>,
                           mut query_target: Query<&mut Transform, (With<Target>, Without<Player>)>,
                           mut timer_target : ResMut<TimerTarget>,
                           time : Res<Time>,
                           mut target_radius : ResMut<TargetRadius>,
                           mut cum_score : ResMut<CumScore>,
                           mut target_num : ResMut<TargetNum>)
{
    let mut player_x = 0.0;
    let mut player_y = 0.0;
    let mut target_x = 0.0;
    let mut target_y = 0.0;
    // get player position from query 
    for player_trans in query_player.iter()
    {
        player_x = player_trans.translation.x;
        player_y = player_trans.translation.y;
    }

    // get target position from query
    for target_trans  in query_target.iter()
    {
        target_x = target_trans.translation.x;
        target_y = target_trans.translation.y;
    }


    let dist_tp = f32::sqrt( (target_x - player_x).powi(2) + (target_y - player_y).powi(2));
    // println!("distance tp : {:?}, timer target time : {:?}, target_radius {:?}", dist_tp, time.elapsed(), target_radius.0);

    // Naive CumScore : 
    cum_score.0 += f32::tanh(dist_tp);
    if dist_tp <= target_radius.0
    {
        timer_target.0.tick(time.delta());

        if timer_target.0.just_finished()
        {
            target_radius.0 /= 2.0;

            if target_radius.0 < TARGET_MIN_RADIUS
            {
                target_num.0 = (target_num.0 + 1) % (TARGET_X.len() as i32);
                target_radius.0 = TARGET_RADIUS_INIT;
                for mut target_trans  in query_target.iter_mut()
                {
                    target_trans.translation.x = TARGET_X[target_num.0 as usize];
                    target_trans.translation.y = TARGET_Y[target_num.0 as usize];
                }
            }
            for mut target_trans in query_target.iter_mut()
            {
                target_trans.scale = Vec3::splat( target_radius.0 / TARGET_RADIUS_INIT);
                // timer_target.0.reset();
            }

        }
    } else 
    {
        timer_target.0.reset();
    }
}
fn set_log_string(mut log_str : ResMut<LogStr>,
    query_player: Query<&Transform, With<Player>>, 
    query_target : Query<&Transform, With<Target>>,
    cum_score : Res<CumScore>, 
    episode_timer : Res<EpisodeTimer>,
    last_cmd_d : Res<LastCmdDisplacement>)
{
    let mut player_pose_x = 0.0;
    let mut player_pose_y = 0.0;
    let mut target_x = 0.0;
    let mut target_y = 0.0;
    let cmd_dx = last_cmd_d.dx;
    let cmd_dy = last_cmd_d.dy;
    let score = cum_score.0;
    let time = episode_timer.0.elapsed().as_secs_f32();
    // println!(" dumps log on time : {:?} ", time);
    // get the player and ball pose 
    
    for transform in query_player.iter()
    {
  
        player_pose_x = transform.translation.x;
        player_pose_y = transform.translation.y;
    }

    for transform in query_target.iter()
    {
        target_x = transform.translation.x;
        target_y = transform.translation.y;
    }

    log_str.0 = format!("{:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2}; {:.2};\n", 
        target_x, target_y, player_pose_x, player_pose_y, cmd_dx, cmd_dy, score, time);
}
