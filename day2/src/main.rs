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
    let mut y_points = 0;
    for line in input_lines {
        if !line.trim().is_empty() {
            let y_move = tr(line.chars().last().unwrap());
            let o_move = tr(line.chars().next().unwrap());
            let result = whowins(y_move, o_move);
            y_points += y_move;
            match result {
                1 => y_points += 6,
                2 => {},
                _ => y_points += 3,
            }
        }
    }
    y_points
}

fn part_two(input_lines: Vec<String>) -> usize {
    let mut y_points = 0;
    for line in input_lines {
        if !line.trim().is_empty() {
            let o_move = tr(line.chars().next().unwrap());
            let result = match line.chars().last().unwrap() {
                'X' => 2,
                'Y' => 0,
                _ => 1,
            };
            if result == whowins(1, o_move) {
                y_points += 1;
            } else if result == whowins(2, o_move) {
                y_points += 2;
            } else if result == whowins(3, o_move) {
                y_points += 3;
            }
            match result {
                1 => y_points += 6,
                2 => {},
                _ => y_points += 3,
            }
        }
    }
    y_points
}

// 1 ROCK, 2 PAPER, 3 SCISSORS
// 0 DRAW, 1 P1, 2 P2
fn whowins(p1:usize, p2:usize) -> usize {
    match p1 {
        1 => {
            match p2 {
                1 => 0,
                2 => 2,
                3 => 1,
                _ => 3,
            }
        },
        2 => {
            match p2 {
                1 => 1,
                2 => 0,
                3 => 2,
                _ => 3,
            }
        }, 
        3 => {
            match p2 {
                1 => 2,
                2 => 1,
                3 => 0,
                _ => 3,
            }
        },
        _ => 3,
    }
}

// Translate - X/A=1, Y/B=2, C/Z=3
fn tr(f_move: char) -> usize {
    let mut ind = 0;
    let mut max = "_ABC".rfind(f_move);
    match max {
        Some(value) => ind += value,
        None        => {},
    }
    max = "_XYZ".rfind(f_move);
    match max {
        Some(value) => ind += value,
        None        => {},
    }
    ind
}
