// file that contain all the functions to manage trajectories
use bevy::prelude::*;
use rand::Rng;
use crate::UPDT;
use crate::env_common::common::*;   

const TRAJECTORY_TO_RUN : Trajectory = Trajectory::Cos;

// const T : f32 = 30.;
const DIR_CHGT : f32 = 1.0;

#[allow(dead_code)]
pub enum Trajectory {
    Linear,
    Random,
    Cos,
    NonMoving,
}


#[derive(Resource)]
pub struct MoveTimer(pub Timer);

# [derive(Resource)]
pub struct DirDrawed(pub bool);

#[derive(Resource)]
pub struct DirTimer(pub Timer);

pub fn linear_dx_trajectory(x: f32, dt: f32, vel_x: &mut f32, width: f32) -> f32
{   
    if f32::abs(x) >= width/2.0 {
        *vel_x = -*vel_x;
    }
    (*vel_x) * dt
}

pub fn cosinus_dx_trajectory(t: f32, dt : f32, width: f32) -> f32
{   
    -width/2.*f32::cos(t)*dt
}



/// generate a random x displacement based on the current velocity and a random number generator
///
/// # Arguments
///
/// * `vel_x` - the current velocity of the object
/// * `rng` - a random number generator
///
/// # Returns
///
/// a random x displacement
pub fn random_dir_trajectory(vel_x: f32, dir_drawed: f32) -> f32
{
    
    // let r = (uniform.sample(rng) * 2) - 1 ;
    (dir_drawed as f32)*vel_x
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
