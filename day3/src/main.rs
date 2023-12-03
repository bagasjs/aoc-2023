use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Token {
    Dot,
    Asterisk,
    Symbol(char),
    Number(i32),
    Ref(i32, i32),
}

struct EngineSchema {
    data: Vec<Token>,
    rows: i32,
    cols: i32,
}

impl EngineSchema {
    fn from_str(data: &str) -> Result<Self, &'static str> {
        let mut result =  Self { data: Vec::new(), rows: 0, cols: 0 };

        for line in data.lines() {
            let line: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < line.len() {
                let ch = line[i];
                match ch {
                    '.' => {
                        result.data.push(Token::Dot);
                        i += 1;
                    }
                    '*' => {
                        result.data.push(Token::Asterisk);
                        i += 1;
                    }
                    _ => {
                        if line[i].is_digit(10) {
                            let mut accum = String::new();
                            while i < line.len() && line[i].is_digit(10) {
                                accum.push(line[i]);
                                i += 1;
                            }
                            if let Ok(num) = accum.parse::<i32>() {
                                let number_row = result.rows;
                                let number_col = (i - accum.len()) as i32;
                                result.data.push(Token::Number(num));
                                for _ in 1..accum.len() {
                                    result.data.push(Token::Ref(number_row, number_col));
                                }
                            } else {
                                return Err("Failed to parse number");
                            }
                        } else {
                            result.data.push(Token::Symbol(ch));
                            i += 1;
                        }
                    },
                }
            }

            result.rows += 1;
            if result.cols == 0 {
                result.cols = i as i32;
            } else if result.cols != (i as i32) {
                return Err("Invalid amount of columns");
            }
        }

        return Ok(result);
    }

    fn index_of(&self, row: i32, col: i32) -> usize {
        return (row * self.cols + col) as usize;
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                match &self.data[self.index_of(row, col)] {
                    Token::Dot => result.push('.'),
                    Token::Asterisk => result.push('*'),
                    Token::Symbol(ch) => result.push(*ch),
                    Token::Number(num) => result.push_str(num.to_string().as_str()),
                    Token::Ref(row, col) => {
                        result.push('(');
                        result.push_str(row.to_string().as_str());
                        result.push_str(col.to_string().as_str());
                        result.push(')');
                    },
                }
                result.push_str(", ");
            }
            result.push('\n');
        }
        return result;
    }

    fn contains(&self, row: i32, col: i32) -> bool {
        return (0 <= row && row < self.rows) && (0 <= col && col < self.cols);
    }

    fn count_sign_nbors(&self, row: i32, col: i32) -> i32 {
        if !self.contains(row, col) {
            return 0;
        }

        let mut result: i32 = 0;

        for drow in -1..=1 {
            for dcol in -1..=1 {
                if drow == 0 && dcol == 0 {
                    continue
                }
                if self.contains(row + drow, col + dcol) {
                    let token = &self.data[self.index_of(row + drow, col + dcol)];
                    match token {
                        Token::Dot => (),
                        Token::Symbol(_) => result += 1,
                        Token::Asterisk => result += 1,
                        Token::Number(_) => (),
                        Token::Ref(_, _) => (),
                    }
                }
            }
        } 

        return result;
    }

    fn number_nbors_info(&self, row: i32, col: i32) -> (usize, i32) {
        if !self.contains(row, col) {
            return (0, 0);
        }

        let mut amount: usize = 0;
        let mut ratio = 1;

        let mut used_number: Vec<i32> = vec![];

        for drow in -1..=1 {
            for dcol in -1..=1 {
                if drow == 0 && dcol == 0 {
                    continue
                }
                if self.contains(row + drow, col + dcol) {
                    let token = &self.data[self.index_of(row + drow, col + dcol)];
                    match token {
                        Token::Number(val) => {
                            if !used_number.contains(val) {
                                amount += 1;
                                ratio *= val;
                                used_number.push(*val);
                            }
                        },
                        Token::Ref(rrow, rcol) => {
                            if let Token::Number(val) = &self.data[self.index_of(*rrow, *rcol)] {
                                if !used_number.contains(val) {
                                    amount += 1;
                                    ratio *= val;
                                    used_number.push(*val);
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }
        }

        return (amount, ratio);
    }
}

fn solve_file(file_path: &str) -> Result<(i32, i32), &'static str> {
    if let Ok(file_content) = fs::read_to_string(file_path) {
        let schema = EngineSchema::from_str(&file_content)?;
        // println!("Schema rows={}, cols={}", schema.rows, schema.cols);
        assert!((schema.data.len() as i32) == (schema.rows*schema.cols));

        let mut current_number = 0;
        let mut sign_nbors_amount = 0;
        let mut sum_of_valid_numbers = 0;

        let mut sum_of_asterisk_gears_ratio = 0;

        for row in 0..schema.rows {
            for col in 0..schema.cols {
                let token = &schema.data[schema.index_of(row, col)];
                match token {
                    Token::Number(value) => {
                        if current_number == 0 {
                            current_number = *value;
                            sign_nbors_amount = schema.count_sign_nbors(row, col);
                        } else {
                            return Err("Unreachable conditions: current_number should be 0 when token is Token::Number");
                        }
                    }
                    Token::Ref(ref_row, ref_col) => {
                        let referenced_token = &schema.data[schema.index_of(*ref_row, *ref_col)];
                        match referenced_token {
                            Token::Number(_) => {
                                sign_nbors_amount += schema.count_sign_nbors(row, col);
                            }
                            _ => {
                                return Err("Unreachable conditions expecting Token::Ref only referencing to Token::Number");
                            },
                        }
                    },
                    Token::Asterisk => {
                        if sign_nbors_amount > 0 {
                            sum_of_valid_numbers += current_number;
                        }
                        current_number = 0;
                        sign_nbors_amount = 0;

                        let (amount, ratio) = schema.number_nbors_info(row, col);
                        if amount == 2 {
                            sum_of_asterisk_gears_ratio += ratio;
                        }
                    },
                    _ => {
                        if sign_nbors_amount > 0 {
                            sum_of_valid_numbers += current_number;
                        }
                        current_number = 0;
                        sign_nbors_amount = 0;
                    },
                }

                // println!("({row},{col}) current_number = {current_number} | nbors = {sign_nbors_amount}");
            }
        }

        return Ok((sum_of_valid_numbers, sum_of_asterisk_gears_ratio))
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
