use std::fs;
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn solve_file(file_path: &str) -> Result<usize, String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut lines = file_content.trim().lines();
    let instructions: Vec<char> = lines.next().ok_or("Failed to parse the instruction of the Map")?
        .trim()
        .chars()
        .collect();

    lines.next();

    let mut maps: HashMap<String, Node> = HashMap::new();
    let mut starts: Vec<String> = Vec::new();

    for line in lines {
        let mut line_it = line.trim().split("=");
        let name = line_it.next().ok_or("Failed to parse the name of a node")?.trim().to_string();
        let mut targets = line_it.next()
            .ok_or(format!("Failed to parse the targets of node {name}"))?
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(",");
        let left = targets.next().ok_or("Failed to parse the left target of a node")?.trim().to_string();
        let right = targets.next().ok_or("Failed to parse the right target of a node")?.trim().to_string();

        if name.ends_with('A') {
            starts.push(name.clone());
        }
        maps.insert(name, Node { left, right });
    }

    let mut every_required_steps: Vec<Vec<usize>> = Vec::new();
    for start in starts.iter() {
        let mut current = start;
        let mut current_required_steps: Vec<usize> = Vec::new();

        let mut last = 0;
        let mut i = 0;
        'calculate_step: loop {
            match instructions[i % instructions.len()] {
                'L' => {
                    current = &maps.get(current).ok_or("Failed to get the current node")?.left;
                },
                'R' => {
                    current = &maps.get(current).ok_or("Failed to get the current node")?.right;
                },
                _ => return Err("Unknown instruction".to_string()),
            }

            if current.ends_with("Z") {
                let step = i - last;
                if current_required_steps.contains(&step) {
                    break 'calculate_step;
                } else {
                    current_required_steps.push(step);
                    last = i;
                }
            }
            i += 1;
        }

        every_required_steps.push(current_required_steps);
    }

    println!("every required steps: {every_required_steps:?}");

    Ok(0)
}

fn main() {
    let file_path = env::args().skip(1).next().expect("Please provide an input file path");
    match solve_file(&file_path) {
        Ok(result) => {
            println!("Result: {result}");
        },
        Err(error_message) => eprintln!("ERROR: {}", error_message),
    }

}
