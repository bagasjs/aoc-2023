use std::env;
use std::fs;

struct SrcToDst {
    src_start: u64,
    dst_start: u64,
    amount: u64,
}

impl SrcToDst {
    fn new(src_start: u64, dst_start: u64, amount: u64) -> Self {
        Self { src_start, dst_start, amount }
    }

    fn in_range(&self, src: u64) -> bool {
        return self.src_start <= src && src < self.src_start + self.amount;
    }
    
    fn dst_of(&self, src: u64) -> Option<u64> {
        if self.in_range(src) {        
            let index = src - self.src_start;
            return Some(self.dst_start + index);
        } else {
            return None;
        }
    }
}

impl std::fmt::Display for SrcToDst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "source ({}..{}), dst ({}..{})",
            self.src_start, self.src_start + self.amount,
            self.dst_start, self.dst_start + self.amount);
        return res;
    }
}

fn dst_of(list_of_src_to_dst: &Vec<SrcToDst>, src: u64) -> u64 {
    for src_to_dst in list_of_src_to_dst {
        if let Some(dst) = src_to_dst.dst_of(src) {
            return dst;
        }
    }
    return src;
}

fn solve_file(file_path: &str) -> Result<(u64, u64), String> {
    let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
    
    let mut seeds = Vec::<u64>::new();
    let mut seed_to_soil = Vec::<SrcToDst>::new();
    let mut soil_to_fertilizer = Vec::<SrcToDst>::new();
    let mut fertilizer_to_water = Vec::<SrcToDst>::new();
    let mut water_to_light = Vec::<SrcToDst>::new();
    let mut light_to_temperature = Vec::<SrcToDst>::new();
    let mut temperature_to_humidity = Vec::<SrcToDst>::new();
    let mut humidity_to_location = Vec::<SrcToDst>::new();

    let file_content = file_content.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(":"))
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    let mut label: Option<&str> = None;
    for content in file_content {
        if content == "seeds" || content.ends_with("map") {
            label = Some(content);
        } else {
            let mut numbers: Vec<u64> = Vec::new();
            for num in content.trim().split_whitespace() {
                numbers.push(num.parse().map_err(|_| "Failed to parse number")?);
            }

            if let Some(label) = label {
                if label == "seeds" {
                    seeds.extend(numbers);
                } else if numbers.len() == 3 {
                    match label {
                        "seed-to-soil map" => seed_to_soil.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        "soil-to-fertilizer map" => soil_to_fertilizer.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        "fertilizer-to-water map" => fertilizer_to_water.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        "water-to-light map" => water_to_light.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        "light-to-temperature map" => light_to_temperature.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        "temperature-to-humidity map" => temperature_to_humidity.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        "humidity-to-location map" => humidity_to_location.push(SrcToDst::new(numbers[1], numbers[0], numbers[2])),
                        _ => (),
                    }
                } else {
                    return Err("Unparseable string".to_string());
                }
            }
        }
    }

    let mut result_location = u64::MAX;
    let mut the_seed = 0; 
    for seed in seeds {
        let mut dst = dst_of(&seed_to_soil, seed);
        dst = dst_of(&soil_to_fertilizer, dst);
        dst = dst_of(&fertilizer_to_water, dst);
        dst = dst_of(&water_to_light, dst);
        dst = dst_of(&light_to_temperature, dst);
        dst = dst_of(&temperature_to_humidity, dst);
        dst = dst_of(&humidity_to_location, dst);
        if dst < result_location {
            the_seed = seed;
            result_location = dst;
        }
    }

    println!("The lowest location is owned by seed {the_seed}");

    Ok((result_location, 0))
}

fn main() {
    let file_path = env::args().skip(1).next().expect("Please provide an input file path");

    match solve_file(&file_path) {
        Ok((part_1, part_2)) => println!("The result is {}, {}", part_1, part_2),
        Err(error_message) => eprintln!("ERROR: {}", error_message),
    }
}
