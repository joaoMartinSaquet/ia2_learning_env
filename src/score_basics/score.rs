// standart deviation of the score when using gaussian
const STD_SCORE : f32 = 100.0;
const PIXEL_DIST_METRIC : f32 = 50.0; 

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
 *               │                 
 *               │                 
 *               │                 
 */
pub fn square_score(x_player : f32, x_folow : f32) -> f32
{
    let dist : f32 = (x_player - x_folow).powi(2);

    if dist < PIXEL_DIST_METRIC
    {
        1.0
    }
    else
    {
        0.0
    }
}