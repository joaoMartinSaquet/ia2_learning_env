
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(200 as u64);

    for i in 0..100 {
        let x : i32 = rng.gen_bool(0.5) as i32 * 2 -1;
        println!("i {} |x : {}", i, x);
    }
}