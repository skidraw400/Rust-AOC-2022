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

    println!("Remember to check, if input.txt HAS blank line in the end");
    println!("Part One: {}", part_one(input_lines.clone()));
    println!("Part Two: {}", part_two(input_lines.clone()));
    
    Ok(())
    
}

fn part_one(input_lines: Vec<String>) -> usize {
    let mut shared = 0;
    for line in input_lines {
        let len = line.trim().len() / 2;
        let string = line.trim().split_at(len);
        let mut priorities = 0;
        let first_half: Vec<char> = string.0.chars().collect();
        for cha in first_half {
            if string.1.contains(cha) {
                priorities = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".rfind(cha).unwrap();
            }
        }
        shared += priorities;
    }
    shared
}

fn part_two(input_lines: Vec<String>) -> usize {
    let mut shared = 0;
    let groups = input_lines.len()/3;
    for group in 0..groups {
        let first: Vec <char> = input_lines[group*3].chars().collect();
        let mut priorities = 0;
        for cha in first {
            if input_lines[group*3+1].contains(cha) && input_lines[group*3+2].contains(cha) {
                priorities = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".rfind(cha).unwrap();
            }   
        }
        shared += priorities;
    }
    shared
}

