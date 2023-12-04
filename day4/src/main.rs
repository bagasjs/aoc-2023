use std::env;
use std::fs;

struct ScratchPad {
    id: i32,
    winning_numbers: Vec<i32>,
    player_numbers: Vec<i32>,
    amount: i32,
}

impl ScratchPad {
    fn new(line: &str) -> Result<Self, &'static str> {
        let elements: Vec<&str> = line.split(":").collect();
        if elements.len() != 2 {
            return Err("Invalid string");
        }

        let card_labels: Vec<&str> = elements[0].trim().split_whitespace().collect();
        if card_labels.len() != 2 {
            return Err("Invalid card label");
        }

        if let Ok(id) = card_labels[1].parse::<i32>() {
            let numbers: Vec<&str> = elements[1].trim().split("|").collect();
            if numbers.len() != 2 {
                return Err("Invalid numbers");
            }

            let mut winning_numbers: Vec<i32> = Vec::new();
            for number in numbers[0].trim().split_whitespace() {
                if let Ok(number) = number.parse::<i32>() {
                    winning_numbers.push(number);
                } else {
                    return Err("Failed to parse winning numbers");
                }
            }

            let mut player_numbers: Vec<i32> = Vec::new();
            for number in numbers[1].trim().split_whitespace() {
                if let Ok(number) = number.parse::<i32>() {
                    player_numbers.push(number);
                } else {
                    return Err("Failed to parse winning numbers");
                }
            }

            return Ok(Self { id, winning_numbers, player_numbers, amount: 1, });
        } else {
            return Err("Failed to parse ScratchPad ID");
        }
    }

    fn amount_of_winning_numbers(&self) -> i32 {
        let mut result = 0;
        for number in &self.player_numbers {
            if self.winning_numbers.contains(number) {
                result += 1;
            }
        }
        return result;
    }

    fn calculate_point(&self) -> i32 {
        let n = self.amount_of_winning_numbers();
        if n == 0 {
            return 0;
        }
        let base: i32 = 2;
        return base.pow((n as u32) - 1);
    }
}

fn solve_file(file_path: &str) -> Result<(i32, i32), &'static str> {
    if let Ok(file_content) = fs::read_to_string(file_path) {
        let mut total_point = 0;
        let mut cards: Vec<ScratchPad> = Vec::new();
        for line in file_content.lines() {
            let scratch_pad = ScratchPad::new(line.trim())?;
            total_point += scratch_pad.calculate_point();
            cards.push(scratch_pad);
        }

        let mut total_amount = 0;
        for i in 0..cards.len() {
            let amount_of_winning_numbers = cards[i].amount_of_winning_numbers();
            for _ in 0..cards[i].amount {
                for j in 0..amount_of_winning_numbers {
                    let index = cards[i].id + (j as i32);
                    if index < (cards.len() as i32) {
                        cards[index as usize].amount += 1;
                    }
                }
                total_amount += 1;
            }
        }

        return Ok((total_point, total_amount));
    } else {
        return Err("Failed to open file content");
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
