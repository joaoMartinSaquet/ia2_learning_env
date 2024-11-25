use core::f32;
use bevy::prelude::*;
use crate::components::env_component::*;
use crate::ressources::env_ressources::*;

use crate::control::control::*;


pub fn move_player(mut query: Query<(&mut Transform, &NameComponent)>,
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