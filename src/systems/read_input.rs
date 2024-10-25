use bevy::prelude::{Query, ResMut, Transform};
use crate::components::env_component::{Position, Velocity, NameComponent};
use std::fs::File;
use std::io::Read;
use crate::ressources::input_ressources::{self, FileInput};

const INPUT_FILE : &str = "input/linear_input.in";

pub fn read_input_from_file(mut file_input : ResMut<FileInput>)
{
    let mut file: File = File::open(INPUT_FILE).unwrap();
    let mut content : String = String::new();

    file.read_to_string(&mut content).unwrap();
    let v : Vec<String> = content.split("\n").map(|x| x.to_string()).collect();
    file_input.0 = v;
}

