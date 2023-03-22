use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let file = File::open("./input.txt")
        .expect("file not found!");
    let  buf_reader = BufReader::new(file);

    let mut input_lines = Vec::new();

    for line in buf_reader.lines() {
        input_lines.push(line?);
    }

    println!("running, file found");
    
    Ok(())

}
