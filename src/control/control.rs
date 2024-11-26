// file that handle control from several sources 
use bevy::prelude::*;
use bevy::prelude::ResMut;
use bevy::input::mouse::MouseMotion;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use regex::Regex;

use crate::env_common::common::*;
use crate::*;

/// COORDINATE SYSTEM of the Mouse
// Origin (0, 0)
//    ●────────── x-axis
//    |
//    |
//    |
//    ↓
// y-axis
const DY_FACTOR : f32 = -1.0;

// last displacement given by a controller (mouse, file, networks etc...)
#[derive(Resource)]
pub struct LastCmdDisplacement
{
    pub dx: f32,
    pub dy: f32
}


pub fn mouse_control(mut mouse_motion: EventReader<MouseMotion>,
    mut last_mouse_movement : ResMut<LastCmdDisplacement>)
{   

let mut dx = 0.0; 
let mut dy = 0.0;
for ev in mouse_motion.read() {

dx = ev.delta.x;
dy = ev.delta.y;

}
last_mouse_movement.dx = dx;
last_mouse_movement.dy = DY_FACTOR*dy;    
// write_to_file_for_now(&mut query); TODO
}

pub fn controller_choice(mut keyboard_input_events: EventReader<KeyboardInput>,
    state: Res<State<ControllerState>>,
    mut next_state: ResMut<NextState<ControllerState>>) {


    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
        // println!("Changing controller state  {:?} event keycode : {:?}", state.get(), event.key_code);
            match state.get() {
                ControllerState::Mouse => match event.key_code {
                                            KeyCode::KeyF => next_state.set(ControllerState::InputFile),
                                            KeyCode::KeyA => next_state.set(ControllerState::Sub),
                                            _ => next_state.set(ControllerState::Mouse),}


                ControllerState::InputFile => match event.key_code {
                                                KeyCode::KeyM => next_state.set(ControllerState::Mouse),
                                                KeyCode::KeyA => next_state.set(ControllerState::Sub),
                                                _ => next_state.set(ControllerState::InputFile),}

                ControllerState::Sub => match event.key_code {
                                                KeyCode::KeyM => next_state.set(ControllerState::Mouse),
                                                KeyCode::KeyF => next_state.set(ControllerState::InputFile),
                                                _ => next_state.set(ControllerState::Sub),}

            }
    // _ => next_state.set(ControllerState::Mouse)
        }
    }
}

#[tokio::main]
pub async fn get_cmd_from_sub(mut sub_socket : ResMut<SubSocketRessource>, 
                              mut last_cmd : ResMut<LastCmdDisplacement>)
{


    let m = sub_socket.0.recv().await;
    // let m = sub_socket.0.monitor();
    println!("message : {:?} ", m);

    if m.is_ok()
    {   
        let data  =  String::from_utf8(m.unwrap().get(0).unwrap().to_vec()).unwrap();
        let re = Regex::new(r"mdx:\s*([-\d.]+);\s*mdy:\s*([-\d.]+);").unwrap();
        if let Some(captures) = re.captures(&data) {
            if let (Some(mdx_match), Some(mdy_match)) = (captures.get(1), captures.get(2)) {
                // Parse the matched values into floats
                let mdx: f64 = mdx_match.as_str().parse().unwrap();
                let mdy: f64 = mdy_match.as_str().parse().unwrap();
    
                println!("mdx: {}, mdy: {}", mdx, mdy);

                last_cmd.dx = mdx as f32;
                last_cmd.dy = mdy as f32;
            }
        }
    }
}

pub fn input_file_control(mut query: Query<(&mut Transform, &NameComponent)>,
                         file_input : Res<FileInput>,
                         mut last_cmd : ResMut<LastCmdDisplacement>,
                         windows: Query<&Window>,
                         episode_timer : Res<EpisodeTimer>)
{
    let width = windows.single().width();
    let index_cmd : f32 = episode_timer.0.elapsed().as_secs_f32() /(UPDT as f32);

    
    // print!("index_cmd {:?} ", index_cmd as usize);
    
    let cmd = &file_input.0[index_cmd as usize].split(";").collect::<Vec<&str>>();
    // println!("cmd from file {:?} ", file_input.0[index_cmd as usize]);
    let dx =  cmd[0].to_string().parse::<f32>().expect("parsed error for dx");
    let dy = 0.0;
    for (mut transform, name) in query.iter_mut()
    {
        if name.0 == "player".to_string()
        {   

            // don't move the player if it's out of bounds
            if f32::abs(transform.translation.x + dx) <= width/2.0
            {
                transform.translation.x += dx;
            }
            // transform.translation.y += ev.delta.y;

        }
    }
    last_cmd.dx = dx;
    last_cmd.dy = dy;    
}


