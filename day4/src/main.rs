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

    println!("Part One: {}", part_one(input_lines.clone()));
    println!("Part Two: {}", part_two(input_lines.clone()));
    
    Ok(())
    
}

fn part_one(input_lines: Vec<String>) -> usize {
    for line in input_lines {
        let inputline = line.split(",");
    }
    0
}

fn part_two(input_lines: Vec<String>) -> usize {
    0
}

