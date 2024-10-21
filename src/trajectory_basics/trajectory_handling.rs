// file that contain all the functions to manage trajectories

use rand::{rngs::{StdRng, ThreadRng}, thread_rng, SeedableRng};
use rand_distr::{Distribution, Normal};

// impact on the pob law:
// - MU_DX : if different from 0, the x displacement will have a bias towards positive or negative values
// - SIGMA_DX : a higher value will result in a higher spread of the x displacement, i.e. a less predictable pob law
// - SEED : a change in the seed will result in a different pob law, i.e. a different set of x displacement
// Conf interval at 99% is calculated by [MU_DX - 2.807*SIGMA_DX; MU_DX + 2.807*SIGMA_DX]
const MU_DX : f32 = 0.0;
const SIGMA_DX : f32 = 10.0;

pub fn linear_dx_trajectory(x: f32, dt: f32, vel_x: &mut f32, width: f32) -> f32
{   
    if f32::abs(x) >= width/2.0 {
        *vel_x = -*vel_x;
    }
    (*vel_x) * dt
}

pub fn random_dx_trajectory(x: f32, width: f32, rng: &mut StdRng) -> f32
{


    let normal = Normal::new(MU_DX, SIGMA_DX).unwrap();
    let r = normal.sample(rng);

    if f32::abs(x + r) >= width/2.0
    {
        0.0
    }
    else
    {
        r 
    }
}

