use core::str;

use crate::components::env_component::NameComponent;
use crate::ressources::env_ressources::{CumScore, EpisodeTimer};
use bevy::prelude::{Query, Res, ResMut, Transform};
use zeromq::{Socket, SocketSend, ZmqMessage};


use crate::control::control::*;
use crate::env_common::common::*;

const SERVER : &str = "tcp://127.0.0.1";   
const LOG_PORT : &str = "5556";
const CMD_PORT : &str = "5560";

const LOG_TOPIC : &str = "GameData/";
const CMD_TOPIC : &str = "";

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
pub async fn publish_log(query: Query<(&Transform, &NameComponent)>, 
                         cum_score : Res<CumScore>, 
                         episode_timer : Res<EpisodeTimer>,
                         mouse_d : Res<LastCmdDisplacement>,
                         mut pub_socket : ResMut<PubSocketRessource>)
{
    let mut player_pose_x = 0.0;
    let mut player_pose_y = 0.0;
    let mut ball_pose_x = 0.0;
    let mut ball_pose_y = 0.0;
    let mouse_dx = mouse_d.dx;
    let mouse_dy = mouse_d.dy;
    let score = cum_score.0;
    let time = episode_timer.0.elapsed().as_secs_f32();


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

    let mut m: ZmqMessage = ZmqMessage::from(LOG_TOPIC);
    if !episode_timer.0.finished()
    {
        let log_str = format!("bx : {:.2}; by : {:.2}; px : {:.2}; py : {:.2}; mdx : {:.2}; mdy : {:.2}; score : {:.2}; t : {:.2};", 
        ball_pose_x, ball_pose_y, player_pose_x, player_pose_y, mouse_dx, mouse_dy, score, time);
        m.push_back(log_str.as_bytes().to_vec().into());
        
        // println!("send message {:?}", m);
        // ignore if there is a problem
        let e = pub_socket.0.send(m).await;

        if e .is_err()
        {
            println!("Error while sending message");
        }
    }
}


