use bevy::prelude::*;
use ia2_learning_env::BounceBall;


fn main() {
    print!("Hello, world!");
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(1.0, 1.0,1.0)));
    app.add_plugins(DefaultPlugins);
    app.add_plugins(BounceBall);
    app.run();

}