use std::env;
use std::fs;

fn solve_part_1(file_content: &str) -> Result<u64, String> {
    let lines: Vec<&str> = file_content.lines().collect();
    let times: Vec<u64> = lines[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<u64> = lines[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if times.len() != distances.len() {
        return Err("Failed to parse string".to_string());
    }

    let mut result_of_part_1 = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let record_distance = distances[i];
        // println!("Race time: {race_time}, Target: {record_distance}");
        let mut winning_count = 0;
        for speed in 0..=race_time {
            let remaining_time = race_time - speed;
            let achieved_distance = remaining_time * speed;
            // print!("Remaining time = {remaining_time}, Achieved Distance = {achieved_distance}");
            if achieved_distance > record_distance {
                // print!(" Winning");
                winning_count += 1;
            }
            // println!();
        }
        // println!("{i}: {winning_count}");
        result_of_part_1 *= winning_count;
    }

    return Ok(result_of_part_1);
}


fn solve_part_2(file_content: &str) -> Result<u64, String> {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut race_time = 0;
    let mut target_distance = 0;
    for line in lines {
        let number = line.split(":")
            .skip(1)
            .next().ok_or("Failed to parse string")?
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("");
        let number: u64 = number.parse().map_err(|_| "Failed to parse number")?;
        race_time = if race_time == 0 { number } else { race_time };
        target_distance = if race_time != 0 { number } else { 0 };
    }

    let mut winning_count = 0;
    for speed in 0..=race_time {
        let remaining_time = race_time - speed;
        let achieved_distance = remaining_time * speed;
        // print!("Remaining time = {remaining_time}, Achieved Distance = {achieved_distance}");
        if achieved_distance > target_distance {
            // print!(" Winning");
            winning_count += 1;
        }
        // println!();
    }
    // println!("{i}: {winning_count}");

    Ok(winning_count)
}

fn solve_file(file_path: &str) -> Result<(u64, u64), String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let result_of_part_1 = solve_part_1(&file_content)?;
    let result_of_part_2 = solve_part_2(&file_content)?;

    return Ok((result_of_part_1, result_of_part_2));
}

fn main() {
    let file_path = env::args().skip(1).next().expect("Please provide an input file path");

    match solve_file(&file_path) {
        Ok((part_1, part_2)) => {
            println!("Result of part 1: {part_1}");
            println!("Result of part 2: {part_2}");
        },
        Err(error_message) => eprintln!("ERROR: {}", error_message),
    }
}
