use bevy::prelude::*;
use ia2_learning_env::BounceBall;

// Set the Fixed Timestep interval to 250 milliseconds

fn main() {
    print!("Hello, world!");
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)));
    app.insert_resource(Time::<Fixed>::from_seconds(0.0001));
    app.add_plugins(DefaultPlugins);
    app.add_plugins(BounceBall);
    
    app.run();

}