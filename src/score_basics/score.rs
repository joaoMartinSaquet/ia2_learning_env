use bevy::prelude::Resource;   


use crate::*;
// standart deviation of the score when using gaussian
const STD_SCORE : f32 = 100.0;
const PIXEL_DIST_METRIC : f32 = 50.0; 
const GSCORE : bool = false;


// Ressources
#[derive(Resource)]
pub struct CumScore(pub f32);

// Components
#[derive(Component)]
pub struct ScoreTxt;


/*                │                   
**                │ 1.0                   
**              xxxxx                 
**             xx │ xx                
**            xx  │  xx               
**            x   │   xx              
**           xx   │    xx             
** xxxxxxxxxxx    │     xxxxxxxxxx    
** ───────────────┼───────────────────
*/                                    
pub fn gaussian_score(x_player : f32, x_folow : f32) -> f32
{
    f32::exp(-(x_folow - x_player).powi(2)/STD_SCORE)
}

/*
 *               ▲                 
 *               │                 
 *               │                 
 *            ┌──┼──┐              
 *            │  │  │              
 *            │  │  │              
 *            │  │  │              
 * ───────────┘  │  └───────────── 
 * ──────────────┼────────────────►         
 */
pub fn square_score(x_player : f32, x_folow : f32) -> f32
{

    let dist : f32 = f32::abs(x_player - x_folow);
    // println!("dist : {:?} ", dist);
    if dist < PIXEL_DIST_METRIC
    {
        1.0
    }
    else
    {
        0.0
    }
}

/// Calculate the score of the player based on its position and the position of the follow object
/// 
/// The score is calculated as a function of the distance between the player and the follow object.
/// The function used is determined by the variable `GSCORE`.
/// If `GSCORE` is true, the score is calculated using a Gaussian function.
/// If `GSCORE` is false, the score is calculated using a square function.
/// The score is then added to the cumulative score and displayed on screen.
pub fn score_metric(query: Query<(&Transform, &NameComponent)>,
                    mut query_text: Query<&mut Text, With<ScoreTxt>>,
                    mut cumscore : ResMut<CumScore>,)
{

    let mut x_player = 0.0;
    let mut x_folow = 0.0;
    // println!("score_metrics on time : {:?} ", time.0.elapsed().as_secs_f32());
    for (transform, name) in query.iter()
    {
        if name.0 == "follow object".to_string()
        {
            x_folow = transform.translation.x;
        }
        if name.0 == "player".to_string()
        {
            x_player = transform.translation.x;
        }        
    }   

    let score ;
    // + eps to avoid division by zero
    // let score = 1./(f32::abs(x_folow - x_player) + 0.01);
    if GSCORE 
    {
        score = gaussian_score(x_player, x_folow);
    }else {
        score = square_score(x_player, x_folow);
    }
    //

    // println!("score {:?} ", score);
    
    cumscore.0 += score;
    let disp_score = cumscore.0;
    
    for mut text in query_text.iter_mut()
    {
        text.sections[1].value = format!("{disp_score:.2}");
        // println!("score {:?}", score);
    }
    
}

pub fn displays_cum_score(cum_score : Res<CumScore>,)
{
    println!("total score is : {:?}", cum_score.0)

}