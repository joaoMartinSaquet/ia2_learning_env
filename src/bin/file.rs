use std::fs::File;
use std::io::Read;



fn main() {
    let mut file: File = File::open("logs/linear_example.log").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    let v : Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    println!("readed v {:?}",v);
    println!("len v {:?}",v.len());
}