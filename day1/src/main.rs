use std::fs;
use std::env;

fn solve_file(file_path: &str) -> std::io::Result<i32> {
    let file_content = fs::read_to_string(file_path)?;
    let number_words = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ];

    let mut result = 0;
    for (_i, line) in file_content.split('\n').enumerate() {
        let mut first: i32 = 0;
        let mut last: i32 = 0;
        let mut accum = String::new();

        for ch in line.chars() {
            if ch.is_numeric() {
                last = ch.to_digit(10).expect("This should not be happened") as i32;
                accum = String::new();
            } else if ch.is_alphabetic() {
                accum.push(ch);
                'check_word: for (i, word) in number_words.iter().enumerate() {
                    if accum.ends_with(word) {
                        last = 1 + i as i32;
                        break 'check_word;
                    }
                }
            }

            first = if first == 0 { last } else { first };
        }

        if first != 0 {
            result += first * 10 + last;
        }
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file path");
        return
    }
    let result = solve_file(&args[1]).expect("Failed to solve file due to invalid file");
    println!("The result is {}", result);
}
