use std::env;
use std::fs;

struct EngineSchema {
    data: Vec<String>,
    rows: i32,
    cols: i32,
}

impl EngineSchema {
    fn from_str(data: &str) -> Self {
        let mut result =  Self { data: Vec::new(), rows: 0, cols: 0 };
        for line in data.lines() {
            let line_len = line.len() as i32;
            if result.cols < line_len {
                result.cols = line_len;
            }
            result.data.push(line.to_string());
        }
        result.rows = result.data.len() as i32;
        return result
    }

    fn contains(&self, row: i32, col: i32) -> bool {
        return (0 <= row && row < self.rows) && (0 <= col && col < self.cols);
    }

    fn at(&self, row: i32, col: i32) -> Option<char> {
        if !self.contains(row, col) {
            return None;
        }
        let line = &self.data[row as usize];
        let result = line.chars().collect::<Vec<char>>()[col as usize];
        Some(result)
    }

    fn count_sign_nbors(&self, row: i32, col: i32) -> i32 {
        if row >= self.rows && col >= self.cols {
            return 0;
        }

        let mut result: i32 = 0;

        // println!("Checking nbors of {row},{col}");
        for drow in -1..=1 {
            for dcol in -1..=1 {
                if drow == 0 && dcol == 0 {
                    continue
                }
                // print!("Nbor of {},{} is ", row + drow, col + dcol);
                if let Some(ch) = self.at(row + drow, col + dcol) {
                    // print!("{ch}");
                    if !ch.is_digit(10) && ch != '.' {
                        result += 1;
                    }
                }
                // println!();
            }
        } 
        // println!("Checking nbors of {row},{col} = {result}");

        return result;
    }
}

fn solve_file(file_path: &str) -> Result<(i32, i32), &'static str> {
    if let Ok(file_content) = fs::read_to_string(file_path) {
        let schema = EngineSchema::from_str(&file_content);
        println!("Schema rows={}, cols={}", schema.rows, schema.cols);
        let mut sum_of_valid_numbers = 0;
        let mut accum = String::new();
        let mut accum_sign_nbors = 0;

        for row in 0..schema.rows {
            for col in 0..schema.cols {
                if let Some(ch) = schema.at(row, col) {
                    if ch.is_digit(10) {
                        accum.push(ch);
                        accum_sign_nbors += schema.count_sign_nbors(row, col);
                    } else {
                        if !accum.is_empty() {
                            println!("Accumulator = {accum}, Nbors = {accum_sign_nbors}");
                            if let Ok(num) = accum.parse::<i32>() {
                                if accum_sign_nbors > 0 {
                                    sum_of_valid_numbers += num;
                                    accum_sign_nbors = 0;
                                }
                            } else {
                                return Err("Failed to parse number");
                            }

                            accum = String::new();
                        }
                    }
                }
            }

            if !accum.is_empty() {
                println!("Accumulator = {accum}, Nbors = {accum_sign_nbors}");
                if let Ok(num) = accum.parse::<i32>() {
                    if accum_sign_nbors > 0 {
                        sum_of_valid_numbers += num;
                        accum_sign_nbors = 0;
                    }
                } else {
                    return Err("Failed to parse number");
                }

                accum = String::new();
            }
        }

        return Ok((sum_of_valid_numbers, 0))
    } else {
        return Err("Failed to read into file");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file path");
        return
    }

    match solve_file(&args[1]) {
        Ok((part_1, part_2)) => println!("The result is {}, {}", part_1, part_2),
        Err(error_message) => eprintln!("ERROR: {}", error_message),
    }
}
