use bevy::prelude::*;
use ia2_learning_env::BounceBall;

// Set the Fixed Timestep interval to 250 milliseconds

fn main() {
    // print!("Hello, world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(BounceBall);
    
    app.run();

}