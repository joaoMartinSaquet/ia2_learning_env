// use bevy::prelude::Color;
// use bevy::prelude::Camera2dBundle;
// use bevy::sprite::{MaterialMesh2dBundle, meshes::Circle, Mesh2dHandle};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::color::palettes::basic::{RED, BLACK};
use crate::ressources::env_ressources::MoveTimer;
use crate::components::env_component::{Velocity, Name};
use crate::RunningState;

const BALL_RADIUS : f32 = 10.0;
const ELASTIC_COEF : f32 = 0.7;
const ACCEL_TIME : f32 = 1.0;

pub fn add_bouncing_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let c: Circle = Circle{radius:BALL_RADIUS};
    // spawn ball to follow

    // let ball = BallBundle::new(&mut meshes, &mut materials, Color::from(RED));
    // println!("visibility {:?}", ball.visibility);
    // commands.spawn( ball );  

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(BALL_RADIUS)).into(),
        material: materials.add(Color::from(RED)),
        transform: Transform::from_xyz( 0.0,
            100.0,
            0.0,
        ),
        ..default()
        }, 
        Velocity {dx: 0.0, dy: -10.0},         Name("bouncing_ball".to_string() ) )

    );
    

    // spawn the ground
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(1000., 5.)).into(),
        material: materials.add(Color::from(BLACK)),
        transform: Transform::from_xyz(
            0.0,
            -5.0,
            0.0,
        ),
        ..default()
    });

}

pub fn spawn_env_camera(mut commands: bevy::prelude::Commands)
{
    commands
    .spawn(Camera2dBundle::default());// .insert(IA2learningEnvPlugin);
}


pub fn move_ball(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,)
{

    for (mut transform, ball_velocity) in query.iter_mut() {
        // println!("translate {:?}", transform);
        transform.translation.x += ball_velocity.dx * time.delta_seconds();
        transform.translation.y += ball_velocity.dy * time.delta_seconds();
    }

}

pub fn ball_dyn_handling(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>,
    state: Res<State<RunningState>>
) {

    // println!("Running state: {:?}", state.get());
    // println!("Elapsed seconds: {}", time.delta_seconds());
    let dt = time.delta_seconds()*ACCEL_TIME;
    // println!("delta time {:?}", dt);
     
    for (mut transform, mut ball_velocity) in query.iter_mut() {

        let _x = transform.translation.x;
        let y = transform.translation.y;
        // println!("position of the ball {:?} {:?}", _x, y);
        if y <= BALL_RADIUS && ball_velocity.dy <= 0.0 {
            // println!("Ball hit on the ground with velovity {:?}", ball_velocity.dy);
            ball_velocity.dy = -ELASTIC_COEF*ball_velocity.dy;
                
        }else {
            ball_velocity.dy = ball_velocity.dy - 9.81*dt;
        }
        transform.translation.y = transform.translation.y + ball_velocity.dy * dt;        

    }
}

pub fn command_desc_text(mut commands: bevy::prelude::Commands, asset_server: Res<AssetServer>){
    
    commands.spawn(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "To run / pause the sim, press {s}",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Thin.ttf"),
                font_size: 50.0,
                color : Color::BLACK,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }));
}
