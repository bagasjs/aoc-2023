use std::env;
use std::fs;
use std::ops::Range;

enum Intersection {
    None, Equal, In, Out, Left, Right
}

trait Intersect {
    fn intersect_with(&self, other: &Self) -> Intersection;
}

impl<T> Intersect for Range<T> 
    where T: PartialOrd
{
    // 1..50 50..100
    fn intersect_with(&self, other: &Self) -> Intersection {
        if other.end == self.end && other.start == self.start {
            Intersection::Equal
        } else if other.start < self.start && self.end < other.end {
            Intersection::In
        } else if self.start <= other.start && other.end < self.end {
            Intersection::Out
        } else if other.start <= self.start && self.start < other.end {
            Intersection::Left
        } else if other.start <= self.end && self.end < other.end {
            Intersection::Right
        } else {
            Intersection::None
        }
    }
}

#[derive(Debug)]
struct SrcDst {
    src: Range<i64>,
    dst: Range<i64>,
}

impl SrcDst {
    fn new(src: Range<i64>, dst: Range<i64>) -> Self {
        Self { src, dst, }
    }

    fn contains(&self, src: i64) -> bool {
        return self.src.start <= src && src <= self.src.end;
    }

    fn is_valid(&self) -> bool {
        return (self.src.end - self.src.start) == (self.dst.end - self.dst.start);
    }

    fn update_on_intersect(&mut self, other: &SrcDst) -> bool {
        if !other.is_valid() {
            return false;
        }

        let mut intersection_occured = false;

        match self.src.intersect_with(&other.src) {
            Intersection::None => (),
            Intersection::Equal | Intersection::In => intersection_occured = true,
            Intersection::Out => {
                self.src.start = other.src.start;
                self.src.end = other.src.end;
                self.dst.start = other.dst.start;
                self.dst.end = other.dst.end;
                intersection_occured = true;
            },

            Intersection::Left => {
                self.dst.start = other.dst.start;
                self.src.start = other.src.start;
                intersection_occured = true;
            },
            Intersection::Right => {
                self.src.end = other.src.end;
                self.dst.end = other.dst.end;
                intersection_occured = true;
            },
        }

        // match self.dst.intersect_with(&other.dst) {
        //     Intersection::None => (),
        //     Intersection::Equal | Intersection::In => {
        //         intersection_occured = true;
        //     }
        //     Intersection::Out => {
        //         self.dst.start = other.dst.start;
        //         self.dst.end = other.dst.end;
        //         intersection_occured = true;
        //     },

        //     Intersection::Left => {
        //         self.dst.start = other.dst.start;
        //         intersection_occured = true;
        //     },
        //     Intersection::Right => {
        //         self.dst.end = other.dst.end;
        //         intersection_occured = true;
        //     },
        // }

        return intersection_occured;
    }

    fn dst_of(&self, src: i64) -> Option<i64> {
        if self.contains(src) {
            let index = src - self.src.start;
            return Some(self.dst.start + index);
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct SrcDstList {
    data: Vec<SrcDst>
}

impl SrcDstList {
    fn new() -> Self {
        Self { data: Vec::with_capacity(32) } 
    }

    fn add(&mut self, item: SrcDst) {
        for srcdst in self.data.iter_mut() {
            if srcdst.update_on_intersect(&item) {
                return;
            }
        }
        self.data.push(item)
    }

    fn dst_of(&self, src: i64) -> i64 {
        for srcdst in self.data.iter() {
            if let Some(dst) = srcdst.dst_of(src) {
                return dst;
            }
        }
        return src;
    }
}

#[derive(Debug)]
struct Almanac {
    seed_to_soil: SrcDstList,
    soil_to_fertilizer: SrcDstList,
    fertilizer_to_water: SrcDstList,
    water_to_light: SrcDstList,
    light_to_temperature: SrcDstList,
    temperature_to_humidity: SrcDstList,
    humidity_to_location: SrcDstList,
}

impl Almanac {
    fn new() -> Self {
        Self { 
            seed_to_soil: SrcDstList::new(),
            soil_to_fertilizer: SrcDstList::new(),
            fertilizer_to_water: SrcDstList::new(),
            water_to_light: SrcDstList::new(),
            light_to_temperature: SrcDstList::new(),
            temperature_to_humidity: SrcDstList::new(),
            humidity_to_location: SrcDstList::new(),
        }
    }

    fn location_of_seed(&mut self, seed: i64) -> i64 {
        let mut location = seed;
        location = self.seed_to_soil.dst_of(location);
        location = self.soil_to_fertilizer.dst_of(location);
        location = self.fertilizer_to_water.dst_of(location);
        location = self.water_to_light.dst_of(location);
        location = self.light_to_temperature.dst_of(location);
        location = self.temperature_to_humidity.dst_of(location);
        location = self.humidity_to_location.dst_of(location);
        return location;
    }

    fn find_lowest_location_from_seed_range(&mut self, range: Range<i64>) -> (i64, i64) {
        let mut result_seed = 0i64;
        let mut result_location = i64::MAX;
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

fn solve_file(file_path: &str) -> Result<(i64, i64), String> {
    let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;
    
    let mut seeds = Vec::<i64>::new();
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
            let mut numbers: Vec<i64> = Vec::new();
            for num in content.trim().split_whitespace() {
                numbers.push(num.parse().map_err(|_| "Failed to parse number")?);
            }

            if let Some(label) = label {
                if label == "seeds" {
                    seeds.extend(numbers);
                } else if numbers.len() == 3 {
                    let dst = numbers[0];
                    let src = numbers[1];
                    let size = numbers[2] - 1;
                    let src_dst = SrcDst::new(src..src+size, dst..dst+size);

                    match label {
                        "seed-to-soil map" => almanac.seed_to_soil.add(src_dst),
                        "soil-to-fertilizer map" => almanac.soil_to_fertilizer.add(src_dst),
                        "fertilizer-to-water map" => almanac.fertilizer_to_water.add(src_dst),
                        "water-to-light map" => almanac.water_to_light.add(src_dst),
                        "light-to-temperature map" => almanac.light_to_temperature.add(src_dst),
                        "temperature-to-humidity map" => almanac.temperature_to_humidity.add(src_dst),
                        "humidity-to-location map" => almanac.humidity_to_location.add(src_dst),
                        _ => return Err("WTF".to_string()),
                    }
                } else {
                    return Err("Unparseable string".to_string());
                }
            }
        }
    }

    // part 1
    let mut part_1_result = i64::MAX;
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
    let mut part_2_result = i64::MAX;
    let mut part_2_seed = 0;
    for chunk in seeds.chunks(2) {
        let start = chunk.get(0).ok_or("Failed to get the start of the seed range")?.clone();
        let amount = chunk.get(1).ok_or("Failed to get the start of the seed range")?.clone();
        println!("{start} {amount}");
        let (seed, location) = almanac.find_lowest_location_from_seed_range(start..start+amount);
        if location < part_2_result {
            part_2_result = location;
            part_2_seed = seed;
        }
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
