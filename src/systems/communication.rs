use crate::components::env_component::NameComponent;
use crate::ressources::env_ressources::{CumScore, EpisodeTimer, LastMouseDisplacement};
use bevy::prelude::{Query, Res, ResMut, Transform};
use zeromq::{ZmqMessage, Socket};
use zeromq::SocketSend;
use crate::ressources::socket_ressources::*;


const SERVER : &str = "tcp://127.0.0.1:5556";   
const TOPIC : &str = "GameData/";

#[tokio::main]
pub async  fn initialize_pub_sub_connection(mut pub_socket : ResMut<PubSocketRessource>,
                                            mut sub_socket : ResMut<SubSocketRessource>)
{   
    // ingoring error because System doesn't handle error 
    let _ = pub_socket.0.bind(SERVER).await;
    println!("publisher Socket binded to {}", SERVER);

    let _ = sub_socket.0.connect(SERVER).await;
    println!("sublisher Socket connected to {}", SERVER);

}

#[tokio::main]
pub async fn publish_log(query: Query<(&Transform, &NameComponent)>, 
                         cum_score : Res<CumScore>, 
                         episode_timer : Res<EpisodeTimer>,
                         mouse_d : Res<LastMouseDisplacement>,
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

    let mut m: ZmqMessage = ZmqMessage::from(TOPIC);
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