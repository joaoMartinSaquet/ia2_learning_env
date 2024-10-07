// file that contain all the functions to manage trajectories


pub fn linear_dx_trajectory(x: f32, dt: f32, vel_x: &mut f32, width: f32) -> f32
{
    if f32::abs(x) >= width/2.0 {
        print!("we change velocity ! ");
        *vel_x = -*vel_x;
    }
    (*vel_x) * dt
}
