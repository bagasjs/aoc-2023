use std::env;
use std::fs;

fn solve_file(file_path: &str) -> Result<(i32, i32), String> {
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| e.to_string())?;

    let mut sequences: Vec<Vec<i32>> = Vec::new();
    for line in file_content.lines() {
        let mut sequence = Vec::<i32>::new();
        for num in line.trim().split_whitespace() {
            sequence.push(num.parse().map_err(|_| "Failed to parse number into i32".to_string())?);
        }
        sequences.push(sequence);
    }
    println!("{sequences:?}");
    return Ok((12, 34));
}

fn main() {
    let file_path = env::args().skip(1).next().expect("Expecting a file path as an arguments");
    match solve_file(&file_path) {
        Ok((part1, part2)) => println!("The answers is ({}, {})", part1, part2),
        Err(msg) => eprintln!("ERROR: {}", msg),
    }
}
