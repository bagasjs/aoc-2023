use std::fs;
use std::env;

#[derive(Clone)]
struct ColorSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl ColorSet {
    fn new() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }

    fn from_str(text: &str) -> Result<Self, &'static str> { let mut result = ColorSet::new();
        for color_element_str in text.split(",") {
            let color_element: Vec<&str> = color_element_str.trim().split_whitespace().collect();

            if color_element.len() == 2 {
                if let Ok(amount) = color_element[0].parse::<i32>() {
                    match color_element[1] {
                        "red" => result.red += amount,
                        "green" => result.green += amount,
                        "blue" => result.blue += amount,
                        _ => (),
                    }
                } else {
                    return Err("Failed to parse the amount of color");
                }
            } else {
                return Err("Invalid color set");
            }
        }

        return Ok(result);
    }

    // fn to_string(&self) -> String {
    //     let mut result = String::new();
    //     result.push_str(&self.red.to_string());
    //     result.push_str(" red, ");
    //     result.push_str(&self.green.to_string());
    //     result.push_str(" green, ");
    //     result.push_str(&self.blue.to_string());
    //     result.push_str(" blue; ");
    //     return result;
    // }
}

struct Game {
    id: i32,
    color_sets: Vec<ColorSet>
}

impl Game {
    fn new() -> Self {
        Self { id: 0, color_sets: vec![], }
    }

    fn inbound(&self, bound: &ColorSet) -> bool {
        for color_set in &self.color_sets {
            if bound.red < color_set.red || bound.green < color_set.green || bound.blue < color_set.blue {
                return false;
            }
        }
        return true;
    }

    fn power(&self) -> i32 {
        let mut least_color_set = ColorSet::new();
        for color_set in &self.color_sets {
            if least_color_set.red < color_set.red {
                least_color_set.red = color_set.red;
            }
            if least_color_set.green < color_set.green {
                least_color_set.green = color_set.green;
            }
            if least_color_set.blue < color_set.blue {
                least_color_set.blue = color_set.blue;
            }
        }
        return least_color_set.red * least_color_set.green * least_color_set.blue;
    }

    fn from_line_str(line: &str) -> Result<Self, &'static str> {
        let elements: Vec<&str> = line.split(":").collect();
        let mut game = Game::new();
        if elements.len() == 2 {
            let label_elements: Vec<&str> = elements[0].split_whitespace().collect();
            if label_elements.len() != 2 {
                return Err("Invalid label element");
            }

            if label_elements[0] != "Game" {
                return Err("Label element doesn't starts with `Game`");
            }

            if let Ok(id) = label_elements[1].parse::<i32>() {
                game.id = id;
            } else {
                return Err("Failed to parse game id");
            }

            for set_element in elements[1].split(";") {
                game.color_sets.push(ColorSet::from_str(set_element)?);
            }

            Ok(game)
        } else {
            return Err("Invalid game line");
        }
    }
}

fn solve_file(file_path: &str, boundary_for_part_1: ColorSet) -> Result<(i32, i32), &'static str> {
    if let Ok(file_content) = fs::read_to_string(file_path) {
        let mut sum_of_id = 0;
        let mut sum_of_power = 0;
        for line in file_content.trim().split("\n") {
            let game = Game::from_line_str(line)?;
            if game.inbound(&boundary_for_part_1) {
                // print!("Game {}: ", game.id);
                // for color_set in &game.color_sets {
                //     print!("{}", color_set.to_string());
                // }
                // println!();
                sum_of_id += game.id;
            }
            sum_of_power += game.power();
        }
        Ok((sum_of_id, sum_of_power))
    } else {
        Err("Failed to read file")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file path");
        return
    }

    let bound = ColorSet { red: 12, green: 13, blue: 14 };
    match solve_file(&args[1], bound) {
        Ok((part_1_answer, part_2_answer)) => {
            println!("The sum of id for part 1 is {part_1_answer}");
            println!("The sum of power for part 2 is {part_2_answer}");
        },
        Err(msg) => {
            eprintln!("{}", msg);
        }
    }
}
