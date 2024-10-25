pub mod components;
pub mod systems;
pub mod ressources;
pub mod trajectory_basics;
pub mod score_basics;


use rand::SeedableRng;
use rand::rngs::StdRng;
use systems::state_handling::controller_choice;
use std::{fs::File, io::Write};
use chrono::{self, Datelike, Timelike}; 


use ressources::env_ressources::{CumScore, EpisodeTimer, LastMouseDisplacement, LogFile, MoveTimer, RandomGen};
use ressources::input_ressources::FileInput;
use systems::{env_systems::*, state_handling::*};
use systems::read_input::read_input_from_file;
use bevy::prelude::*;

// dt of the move timer every 0.05 seconds
const MOVE_DT : f32 = 0.005;

// episodes duration
const EPISODE_DURATION : f32  = 10.0;

// parameters of the normal distribution used in the random trajectory
// MU_DX : mean of the normal distribution, i.e. the mean of the x displacement
// SIGMA_DX : standard deviation of the normal distribution, i.e. the spread of the x displacement
// SEED : seed used to generate the random number, i.e. used to generate the x displacement
const SEED : u64 = 4896484;

// Delta t between two updates : typical 0.02 because it seems that it is mouse dt
// most of the mouse are with a frequence of 125 Hz (0.008 s) or hte game freq ! must check that
const UPDT : f64 = 0.008;

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
}

// #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
// enum MyInputKindSet {
//     Touch,
//     Mouse,
//     Gamepad,
// }


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
        let r: StdRng = StdRng::seed_from_u64(SEED);


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

        app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)))
           .insert_resource(Time::<Fixed>::from_seconds(UPDT))
           .insert_resource(EpisodeTimer(Timer::from_seconds(EPISODE_DURATION, TimerMode::Repeating)))
           .insert_resource(CumScore(0.0))
           .insert_resource(RandomGen(r))
           .insert_resource(LastMouseDisplacement {dx: 0.0, dy: 0.0})
           .insert_resource(LogFile(log_file))
           .insert_resource(FileInput(vec![]))
           .init_state::<RunningState>()
           .init_state::<ControllerState>()
           .add_systems(Startup, setup_env)
           .add_systems(Update, toggle_run_pause)
           .add_systems(FixedUpdate, (run_trajectory).run_if(in_state(RunningState::Running)))
           .add_systems(Update, (mouse_control).run_if(in_state(ControllerState::Mouse)).run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, (input_file_control).run_if(in_state(ControllerState::InputFile)).run_if(in_state(RunningState::Running)))
           .add_systems(FixedUpdate, (score_metric, dumps_log).run_if(in_state(RunningState::Running)))
           .add_systems(Update, episodes_ends)
           .add_systems(OnEnter(RunningState::Ended), displays_cum_score)
           .add_systems(OnEnter(RunningState::Started), restart)
           .add_systems(OnEnter(ControllerState::InputFile), read_input_from_file)
           .add_systems(Update, controller_choice.run_if(in_state(RunningState::Started)));
    
    }
}

