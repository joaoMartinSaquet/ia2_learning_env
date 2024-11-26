use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::menu::menu::OnGameScreen;
use crate::control::control::LastCmdDisplacement;
use crate::{RunningState, TaskState};
// use bevy::prelude::Color::rgb;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Target;


const TARGET_RADIUS_INIT : f32 = 100.0;
const TARGET_COLOR : Color=  Color::srgba(0.636, 0.968, 0.596, 0.3);

// time the player stay inside the target
const TIMER_TARGET_STAY : f32 = 5.0;



#[derive(Resource)]
pub struct TargetRadius(f32);

#[derive(Resource)]
pub struct TimerTarget(Timer);



pub struct TargetSelectionPlugin;
impl Plugin for TargetSelectionPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(TargetRadius(TARGET_RADIUS_INIT))
            .insert_resource(TimerTarget(Timer::from_seconds(TIMER_TARGET_STAY, TimerMode::Repeating)))
           .add_systems(OnEnter(TaskState::TargetSelection), setup_target)
           .add_systems(Update, (move_entity).run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::TargetSelection)))
           .add_systems(FixedUpdate, is_player_in_target.run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::TargetSelection)));
    }
}



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

pub fn is_player_in_target(query_player: Query<&Transform, With<Player>>,
                           mut query_target: Query<&mut Transform, (With<Target>, Without<Player>)>,
                           mut timer_target : ResMut<TimerTarget>,
                           time : Res<Time>,
                           mut target_radius : ResMut<TargetRadius>)
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
    println!("distance tp : {:?}, timer target time : {:?}, target_radius {:?}", dist_tp, timer_target.0.remaining_secs(), target_radius.0);
    if dist_tp <= target_radius.0
    {
        timer_target.0.tick(time.delta());

        if timer_target.0.just_finished()
        {
            target_radius.0 /= 2.0;
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