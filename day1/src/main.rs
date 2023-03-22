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

    println!("Remember to check, if input.txt has blank line in the end");
    println!("Part One: {}", part_one(input_lines.clone()));
    println!("Part Two: {}", part_two(input_lines.clone()));
    
    Ok(())
    
}

fn part_one(input_lines: Vec<String>) -> i32 {
    let mut cal_vec = Vec::new();
    let mut temp = 0;

    for line in input_lines {
        if line.trim().is_empty() {
            cal_vec.push(temp);
            temp = 0;
        } else {
            temp += line.parse::<i32>().unwrap();
        }
    }
    
    let max = cal_vec.iter().max();
    let mut result = 0_i32;
    match max {
        Some(value) => result += value,
        None        => result = 0,
    }
    result
}

fn part_two(input_lines: Vec<String>) -> i32 {
    let mut cal_vec = Vec::new();
    let mut temp = 0;
    
    for line in input_lines {
        if line.trim().is_empty() {
            cal_vec.push(temp);
            temp = 0;
        } else {
            temp += line.parse::<i32>().unwrap();
        }
    }

    cal_vec.sort();
    cal_vec.reverse();
    cal_vec = cal_vec[0..3].to_vec();

    let mut ret = 0_i32;
    for cal in cal_vec {
        ret += cal;
    }
    ret
}
