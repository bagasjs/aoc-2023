use std::env;
use std::fs;
use std::collections::HashMap;
use std::ops::Range;
enum Intersection {
    None, Equal, Inner, Outer, Left, Right
}

trait Intersect {
    fn intersect_with(&self, other: &Self) -> Intersection;
}

impl<T> Intersect for Range<T> 
    where T: PartialOrd
{
    fn intersect_with(&self, other: &Self) -> Intersection {
        if other.end == self.end && other.start == self.start {
            Intersection::Equal
        } else if other.end > self.end && other.start < self.start {
            Intersection::Inner
        } else if self.end > other.end && self.start < other.start {
            Intersection::Outer
        } else if self.end > other.end {
            Intersection::Right
        } else if self.start < other.start {
            Intersection::Right
        } else {
            Intersection::None
        }
    }
}

struct RangeList {
    data: Vec<Range<u64>>
}

impl RangeList {
    fn new() -> Self {
        return Self {
            data: Vec::with_capacity(32),
        }
    }

    fn add(&mut self, range: Range<u64>) {
        for r in self.data.iter_mut() {
            match range.intersect_with(r) {
                Intersection::Equal | Intersection::Inner => return,
                Intersection::Outer => {
                    r.start = range.start;
                    r.end = range.end;
                    return;
                },
                Intersection::Left => {
                    r.start = range.start;
                    return;
                },
                Intersection::Right => {
                    r.end = range.end;
                    return;
                },
                Intersection::None => {
                    continue
                },
            }
        }

        self.data.push(range);
    }
}

#[derive(Debug)]
struct SrcToDst {
    src_start: u64,
    dst_start: u64,
    amount: u64,
}

impl SrcToDst {
    fn new(src_start: u64, dst_start: u64, amount: u64) -> Self {
        Self { src_start, dst_start, amount }
    }

    fn contains(&self, src: u64) -> bool {
        return self.src_start <= src && src < self.src_start + self.amount;
    }
    
    fn dst_of(&self, src: u64) -> Option<u64> {
        if self.contains(src) {        
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

#[derive(Debug)]
struct Almanac {
    seed_to_soil: Vec<SrcToDst>,
    soil_to_fertilizer: Vec<SrcToDst>,
    fertilizer_to_water: Vec<SrcToDst>,
    water_to_light: Vec<SrcToDst>,
    light_to_temperature: Vec<SrcToDst>,
    temperature_to_humidity: Vec<SrcToDst>,
    humidity_to_location: Vec<SrcToDst>,

    seed_to_soil_cache: HashMap<u64, u64>,
    soil_to_fertilizer_cache: HashMap<u64, u64>,
    fertilizer_to_water_cache: HashMap<u64, u64>,
    water_to_light_cache: HashMap<u64, u64>,
    light_to_temperature_cache: HashMap<u64, u64>,
    temperature_to_humidity_cache: HashMap<u64, u64>,
    humidity_to_location_cache: HashMap<u64, u64>,
    seed_to_location_cache: HashMap<u64, u64>,
}

impl Almanac {
    fn new() -> Self {
        Self { 
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),

            seed_to_soil_cache: HashMap::with_capacity(50), soil_to_fertilizer_cache: HashMap::with_capacity(50),
            fertilizer_to_water_cache: HashMap::with_capacity(50),
            water_to_light_cache: HashMap::with_capacity(50),
            light_to_temperature_cache: HashMap::with_capacity(50),
            temperature_to_humidity_cache: HashMap::with_capacity(50),
            humidity_to_location_cache: HashMap::with_capacity(50),
            seed_to_location_cache: HashMap::with_capacity(50),
        }
    }

    fn soil_of(&mut self, seed: u64) -> u64 {
        if self.seed_to_soil_cache.contains_key(&seed) {
            return self.seed_to_soil_cache[&seed];
        } else {
            for seed_to_soil in self.seed_to_soil.iter() {
                if let Some(dst) = seed_to_soil.dst_of(seed) {
                    self.seed_to_soil_cache.insert(seed, dst);
                    return dst;
                }
            }
            return seed;
        }
    }

    fn fertilizer_of(&mut self, soil: u64) -> u64 {
        if self.soil_to_fertilizer_cache.contains_key(&soil) {
            return self.soil_to_fertilizer_cache[&soil];
        } else {
            for soil_to_fertilizer in self.soil_to_fertilizer.iter() {
                if let Some(dst) = soil_to_fertilizer.dst_of(soil) {
                    self.soil_to_fertilizer_cache.insert(soil, dst);
                    return dst;
                }
            }
            return soil;
        }
    }

    fn water_of(&mut self, fertilizer: u64) -> u64 {
        if self.fertilizer_to_water_cache.contains_key(&fertilizer) {
            return self.fertilizer_to_water_cache[&fertilizer];
        } else {
            for fertilizer_to_water in self.fertilizer_to_water.iter() {
                if let Some(dst) = fertilizer_to_water.dst_of(fertilizer) {
                    self.fertilizer_to_water_cache.insert(fertilizer, dst);
                    return dst;
                }
            }
            return fertilizer;
        }
    }

    fn light_of(&mut self, water: u64) -> u64 {
        if self.water_to_light_cache.contains_key(&water) {
            return self.water_to_light_cache[&water];
        } else {
            for water_to_light in self.water_to_light.iter() {
                if let Some(dst) = water_to_light.dst_of(water) {
                    self.water_to_light_cache.insert(water, dst);
                    return dst;
                }
            }
            return water;
        }
    }

    fn temperature_of(&mut self, light: u64) -> u64 {
        if self.light_to_temperature_cache.contains_key(&light) {
            return self.light_to_temperature_cache[&light];
        } else {
            if let Some(temp) = self.light_to_temperature.iter().find(|st| st.contains(light)) {
            }
            for light_to_temperature in self.light_to_temperature.iter() {
                if let Some(dst) = light_to_temperature.dst_of(light) {
                    self.light_to_temperature_cache.insert(light, dst);
                    return dst;
                }
            }
            return light;
        }
    }

    fn humidity_of(&mut self, temperature: u64) -> u64 {
        if self.temperature_to_humidity_cache.contains_key(&temperature) {
            return self.temperature_to_humidity_cache[&temperature];
        } else {
            for temperature_to_humidity in self.temperature_to_humidity.iter() {
                if let Some(dst) = temperature_to_humidity.dst_of(temperature) {
                    self.temperature_to_humidity_cache.insert(temperature, dst);
                    return dst;
                }
            }
            return temperature;
        }
    }

    fn location_of(&mut self, humidity: u64) -> u64 {
        if self.humidity_to_location_cache.contains_key(&humidity) {
            return self.humidity_to_location_cache[&humidity];
        } else {
            for humidity_to_location in self.humidity_to_location.iter() {
                if let Some(dst) = humidity_to_location.dst_of(humidity) {
                    self.humidity_to_location_cache.insert(humidity, dst);
                    return dst;
                }
            }
            return humidity;
        }
    }

    fn location_of_seed(&mut self, seed: u64) -> u64 {
        if self.seed_to_location_cache.contains_key(&seed) {
            println!("Duplication");
            return self.seed_to_location_cache[&seed];
        } else {
            let mut dst = self.soil_of(seed);
            dst = self.fertilizer_of(dst);
            dst = self.water_of(dst);
            dst = self.light_of(dst);
            dst = self.temperature_of(dst);
            dst = self.humidity_of(dst);
            dst = self.location_of(dst);
            self.seed_to_location_cache.insert(seed, dst);
            return dst;
        }
    }

    fn find_lowest_location_from_seed_range(&mut self, range: std::ops::Range<u64>) -> (u64, u64) {
        println!("{range:?}");
        let mut result_seed = 0u64;
        let mut result_location = u64::MAX;
        for seed in range {
            let location = self.location_of_seed(seed);
            if location < result_location {
                result_location = location;
                result_seed = seed;
            }
        }

        return (result_seed, result_location);
    }
}

fn solve_file(file_path: &str) -> Result<(u64, u64), String> {
    let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
    
    let mut seeds = Vec::<u64>::new();
    let mut almanac = Almanac::new();

    let file_content = file_content.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(":"))
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    let mut label: Option<&str> = None;

    // parsing
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
                    let src_to_dst = SrcToDst::new(numbers[1], numbers[0], numbers[2]);
                    match label {
                        "seed-to-soil map" => almanac.seed_to_soil.push(src_to_dst),
                        "soil-to-fertilizer map" => almanac.soil_to_fertilizer.push(src_to_dst),
                        "fertilizer-to-water map" => almanac.fertilizer_to_water.push(src_to_dst),
                        "water-to-light map" => almanac.water_to_light.push(src_to_dst),
                        "light-to-temperature map" => almanac.light_to_temperature.push(src_to_dst),
                        "temperature-to-humidity map" => almanac.temperature_to_humidity.push(src_to_dst),
                        "humidity-to-location map" => almanac.humidity_to_location.push(src_to_dst),
                        _ => (),
                    }
                } else {
                    return Err("Unparseable string".to_string());
                }
            }
        }
    }

    println!("{:?}", almanac);
    // part 1
    let mut part_1_result = u64::MAX;
    let mut part_1_seed = 0;
    for seed in seeds.iter() {
        let seed = seed.clone();
        let dst = almanac.location_of_seed(seed);
        if dst < part_1_result {
            part_1_seed = seed;
            part_1_result = dst;
        }
    }

    // part 2
    let mut part_2_result = u64::MAX;
    let mut part_2_seed = 0;
    for chunk in seeds.chunks(2) {
        let start = chunk.get(0).ok_or("Failed to get the start of the seed range")?.clone();
        let amount = chunk.get(1).ok_or("Failed to get the start of the seed range")?.clone();
        let (seed, location) = almanac.find_lowest_location_from_seed_range(start..start+amount);
        if location < part_2_result {
            part_2_result = location;
            part_2_seed = seed;
        }
        break;
    }

    println!("Seed of part 1: {part_1_seed}");
    println!("Seed of part 2: {part_2_seed}");
    Ok((part_1_result, part_2_result))
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
