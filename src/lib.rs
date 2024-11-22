pub mod components;
pub mod systems;
pub mod ressources;
pub mod trajectory_basics;
pub mod score_basics;


use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use systems::communication::initialize_pub_sub_connection;
use systems::state_handling::controller_choice;
use zeromq::PubSocket;
use std::{fs::File, io::Write};
use chrono::{self, Datelike, Timelike}; 

// zeromq
use zeromq::*;

use ressources::env_ressources::{CumScore, EpisodeTimer, LastCmdDisplacement, 
    LogFile, MoveTimer, RandomGen, DirDrawed, DirTimer};
use ressources::input_ressources::FileInput;
use ressources::socket_ressources::*;
use systems::{env_systems::*, state_handling::*, communication::*, read_input::*, player::*};
use bevy::prelude::*;



// dt of the move timer every 0.05 seconds
const MOVE_DT : f32 = 0.005;

// episodes duration
const EPISODE_DURATION : f32  = 30.;

// parameters of the normal distribution used in the random trajectory
// MU_DX : mean of the normal distribution, i.e. the mean of the x displacement
// SIGMA_DX : standard deviation of the normal distribution, i.e. the spread of the x displacement
// SEED : seed used to generate the random number, i.e. used to generate the x displacement
const SEED : u64 = 200;

// Delta t between two updates : typical 0.02 because it seems that it is mouse dt
// most of the mouse are with a frequence of 125 Hz (0.008 s) or hte game freq ! must check that
const UPDT : f64 = 0.008; // old 0.008

const HEADER_LOG_FILE : &str = "Bx;By;Px;Py;Mdx;Mdy;Score;Time;\n";


#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunningState {
    #[default]
    Started,
    Running,
    Ended,
    Paused,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ControllerState {
    #[default]
    Mouse,
    InputFile,
    Sub,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum NetworkState {
    #[default]
    Unconnected,
    Connected,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum TaskState {
    #[default]
    FollowBall,
    TargetSelection,
}



pub struct BounceBall;
impl Plugin for BounceBall {
    fn build(&self, app: &mut App) {

        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)))
            .insert_resource(Time::<Fixed>::from_seconds(0.0001))
            .init_state::<RunningState>()
            // .add_plugins(EntropyPlugin::<WyRand>::with_seed(seed.to_ne_bytes()))
            .add_systems(Startup,setup_bouncing_ball)
            .insert_resource(MoveTimer(Timer::from_seconds(MOVE_DT, TimerMode::Repeating)))
            .add_systems(FixedUpdate, (ball_dyn_handling).run_if(in_state(RunningState::Running)));       
    }
}

pub struct  LearningEnv;
impl Plugin for LearningEnv
{
    fn build(&self, app: &mut App) {

        // genere a random distribution with a seed
        // let r: StdRng = StdRng::seed_from_u64();
        let r: ChaCha8Rng = ChaCha8Rng::seed_from_u64(SEED);

        // Log file creation
        let date_time = chrono::offset::Local::now(); 
        // Format the date and time as YY_MM_DAY_HH_mm_SS
        let formatted_date = format!(
            "{:02}_{:02}_{:02}_{:02}_{:02}_{:02}",
            date_time.year() % 100,  // Last two digits of the year
            date_time.month(),       // Month (01-12)
            date_time.day(),         // Day of the month (01-31)
            date_time.hour(),        // Hour (00-23)
            date_time.minute(),      // Minute (00-59)
            date_time.second()       // Second (00-59)
        );
        println!("date {:?}",formatted_date);
        let log_file_path = "logs/application".to_owned() + &formatted_date + ".log";
        let mut log_file = File::create(log_file_path).unwrap();
        log_file.write(HEADER_LOG_FILE.as_bytes()).unwrap();

        // Publisher socket creation 
        let log_socket : PubSocket = zeromq::PubSocket::new();
        let cmd_socket : SubSocket = zeromq::SubSocket::new();



        // add basic ressources
        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)))
           .insert_resource(Time::<Fixed>::from_seconds(UPDT))
           .insert_resource(EpisodeTimer(Timer::from_seconds(EPISODE_DURATION, TimerMode::Repeating)))
           .insert_resource(DirTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
           .insert_resource(CumScore(0.0))
           .insert_resource(RandomGen(r))
           .insert_resource(DirDrawed(false))
           .insert_resource(LastCmdDisplacement {dx: 0.0, dy: 0.0})
           .insert_resource(LogFile(log_file))
           .insert_resource(FileInput(vec![]))
           .insert_resource(PubSocketRessource(log_socket))
           .insert_resource(SubSocketRessource(cmd_socket));
        

        // initialize states 
        app
           .init_state::<RunningState>()
           .init_state::<ControllerState>()
           .init_state::<NetworkState>()
           .init_state::<TaskState>();
        
           
        // add systems
        app
            // startup
           .add_systems(Startup, setup_env.run_if(in_state(TaskState::FollowBall)))
            // change running state
           .add_systems(Update, toggle_run_pause)
           // change the networking state
           .add_systems(Update, networking_choice.run_if(in_state(RunningState::Started)))
           // change the controller state
           .add_systems(Update, controller_choice.run_if(in_state(RunningState::Started)))


           // on running systems 
           .add_systems(FixedUpdate, (run_trajectory).run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::FollowBall)).before(score_metric).before(dumps_log))
           .add_systems(FixedUpdate, (input_file_control).run_if(in_state(ControllerState::InputFile)).run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, (score_metric, dumps_log).chain().run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, run_episodes_timer.before(run_trajectory))
           .add_systems(Update, (mouse_control).run_if(in_state(ControllerState::Mouse)).run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, change_direction.run_if(in_state(RunningState::Running)).run_if(in_state(TaskState::FollowBall)))
           .add_systems(FixedUpdate, publish_log.run_if(in_state(NetworkState::Connected)).run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, get_cmd_from_sub.run_if(in_state(NetworkState::Connected)).run_if(in_state(RunningState::Running)).run_if(in_state(ControllerState::Sub)).after(publish_log))
           .add_systems(FixedUpdate, move_player.run_if(in_state(RunningState::Running)).before(dumps_log))

           // on state change systems
           .add_systems(OnEnter(RunningState::Ended), displays_cum_score)
           .add_systems(OnEnter(RunningState::Started), restart)
           .add_systems(OnEnter(ControllerState::InputFile), read_input_from_file)
           .add_systems(OnEnter(NetworkState::Connected), initialize_pub_sub_connection);
    }
}

