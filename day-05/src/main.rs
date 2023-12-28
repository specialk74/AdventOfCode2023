use utils::from_slice_to_vec;

#[derive(Clone)]
struct Map {
    destination: u64,
    source: u64,
    range: u64
}

impl Map {
    fn new (input: &str) -> Self {
        let values = utils::from_slice_to_vec(input);
        Self {
            destination: values[0],
            source: values[1],
            range: values[2]
        }
    }

    fn get(&self, number: u64) -> Option<u64> {
        if number < self.source || number > self.source + self.range {
            return None;
        }
        Some(self.destination + (number - self.source))
    }
}

#[derive(Clone)]
struct Maps {
    values: Vec<Map>,
}

impl Maps {
    fn new () -> Self {
        Self {
            values: Vec::new(),
        }
    }

    fn add(&mut self, input: &str) {
        self.values.push(Map::new(input));
    }

    fn get(&self, number: u64) -> u64 {
        for val in &self.values {
            if let Some(val) = val.get(number) {
                return val;
            }
        }

        number
    }
}

#[derive(Clone)]
struct Seeds {
    start: u64,
    length: u64,
}

impl Seeds {
    fn new(start: u64, length: u64) -> Self {
        Self {
            start,
            length            
        }
    }
}

#[derive(Clone)]
struct Almanac {
    seeds: Vec<u64>,
    seeds_range: Vec<Seeds>,
    seed_to_soil: Maps,
    soil_to_fertilizer: Maps,
    fertilizer_to_water: Maps,
    water_to_light: Maps,
    light_to_temperature: Maps,
    temperature_to_humidity: Maps,
    humidity_to_location: Maps
}

impl Almanac {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            seeds_range: Vec::new(),
            seed_to_soil: Maps::new(),
            soil_to_fertilizer: Maps::new(),
            fertilizer_to_water: Maps::new(),
            water_to_light: Maps::new(),
            light_to_temperature: Maps::new(),
            temperature_to_humidity: Maps::new(),
            humidity_to_location: Maps::new(),
        }
    }

    fn get_min_location(&self) -> u64 {
        let mut min_location = u64::MAX;
        for seed in &self.seeds {
            let soil = self.seed_to_soil.get(*seed);
            let fertilizer = self.soil_to_fertilizer.get(soil);
            let water = self.fertilizer_to_water.get(fertilizer);
            let light = self.water_to_light.get(water);
            let temperature = self.light_to_temperature.get(light);
            let humidity = self.temperature_to_humidity.get(temperature);
            let location = self.humidity_to_location.get(humidity);
            if  location < min_location {
                min_location = location;
            }

        }
        min_location
    }

    fn get_min_location_by_seed(&self, seeds: &Seeds) -> u64 {
        let mut min_location = u64::MAX;

        println!("Start: {} - Length: {}", seeds.start, seeds.length);
        for seed in seeds.start..seeds.start + seeds.length{
            let soil = self.seed_to_soil.get(seed);
            let fertilizer = self.soil_to_fertilizer.get(soil);
            let water = self.fertilizer_to_water.get(fertilizer);
            let light = self.water_to_light.get(water);
            let temperature = self.light_to_temperature.get(light);
            let humidity = self.temperature_to_humidity.get(temperature);
            let location = self.humidity_to_location.get(humidity);
            if  location < min_location {
                min_location = location;
            }
        }
        min_location
    }

    fn get_min_location_range(&self) -> u64 {
        let mut min_location_vec = Vec::new();
        for seeds in &self.seeds_range {
            min_location_vec.push(self.get_min_location_by_seed(seeds));
        }
        *min_location_vec.iter().min().unwrap()
    }
    /*
    fn get_min_location_range_using_thread(&self) -> u64 {
        let mut min_location = Vec::new();
        let mut threar_list = Vec::new();
        for seeds in &self.seeds_range {
            let self_copy = Arc::new(self);
            let handle_internal = thread::spawn(move || 
                self_copy.get_min_location_by_seed(&seeds)
            );
            threar_list.push(handle_internal);
        }

        for handle in threar_list {
            let value = handle.join().unwrap();
            min_location.push(value);
        }

        *min_location.iter().min().unwrap()
    }
    */
}

enum States {
    None,

    SeedToSoil,
    SeedToSoilData,

    SoilToFertilizer,
    SoilToFertilizerData,

    FertilizerToWater,
    FertilizerToWaterData,

    WaterToLight,
    WaterToLightData,

    LightToTemperature,
    LightToTemperatureData,

    TemperatureToHumidity,
    TemperatureToHumidityData,

    HumidityToLocation,
    HumidityToLocationData
}

fn part1(lines: &str) -> u64 {
    let mut result = Almanac::new();
    let mut state = States::None;
    for line in lines.split('\n').filter(|&x| !x.trim().is_empty()) {

        if line.contains("seeds:") {
            let values = line
                                    .trim()
                                    .split(':')
                                    .nth(1)
                                    .unwrap();
            result.seeds = from_slice_to_vec(values);
        }
        else if line.contains("seed-to-soil map:")
        {
            state = States::SeedToSoil;
        }
        else if line.contains("soil-to-fertilizer map:")
        {
            state = States::SoilToFertilizer;
        }
        else if line.contains("fertilizer-to-water map:")
        {
            state = States::FertilizerToWater;
        }
        else if line.contains("water-to-light map:")
        {
            state = States::WaterToLight;
        }
        else if line.contains("light-to-temperature map:")
        {
            state = States::LightToTemperature;
        }
        else if line.contains("temperature-to-humidity map:")
        {
            state = States::TemperatureToHumidity;
        }
        else if line.contains("humidity-to-location map:")
        {
            state = States::HumidityToLocation;
        }

        match state {
            States::SeedToSoil => {state = States::SeedToSoilData;},
            States::SeedToSoilData => {result.seed_to_soil.add(line)}

            States::SoilToFertilizer => {state = States::SoilToFertilizerData;},
            States::SoilToFertilizerData => {result.soil_to_fertilizer.add(line)}

            States::FertilizerToWater => {state = States::FertilizerToWaterData;},
            States::FertilizerToWaterData => {result.fertilizer_to_water.add(line)}

            States::WaterToLight => {state = States::WaterToLightData;},
            States::WaterToLightData => {result.water_to_light.add(line)}

            States::LightToTemperature => {state = States::LightToTemperatureData;},
            States::LightToTemperatureData => {result.light_to_temperature.add(line)}

            States::TemperatureToHumidity => {state = States::TemperatureToHumidityData;},
            States::TemperatureToHumidityData => {result.temperature_to_humidity.add(line)}

            States::HumidityToLocation => {state = States::HumidityToLocationData;},
            States::HumidityToLocationData => {result.humidity_to_location.add(line)}
            _ => {}
        }
    }

    result.get_min_location()
}

fn part2(lines: &str) -> u64 {
    let mut result = Almanac::new();
    let mut state = States::None;
    for line in lines.split('\n').filter(|&x| !x.trim().is_empty()) {

        if line.contains("seeds:") {
            let values = line
                                    .trim()
                                    .split(':')
                                    .nth(1)
                                    .unwrap();
            let seeds_orig = from_slice_to_vec(values);
            let mut seeds: Vec<Seeds> = Vec::new();

            let mut index = 0;
            while index < seeds_orig.len() {
                seeds.push(Seeds::new(seeds_orig[index], seeds_orig[index + 1]));
                index += 2;
            }

            result.seeds_range = seeds;
        }
        else if line.contains("seed-to-soil map:")
        {
            state = States::SeedToSoil;
        }
        else if line.contains("soil-to-fertilizer map:")
        {
            state = States::SoilToFertilizer;
        }
        else if line.contains("fertilizer-to-water map:")
        {
            state = States::FertilizerToWater;
        }
        else if line.contains("water-to-light map:")
        {
            state = States::WaterToLight;
        }
        else if line.contains("light-to-temperature map:")
        {
            state = States::LightToTemperature;
        }
        else if line.contains("temperature-to-humidity map:")
        {
            state = States::TemperatureToHumidity;
        }
        else if line.contains("humidity-to-location map:")
        {
            state = States::HumidityToLocation;
        }

        match state {
            States::SeedToSoil => {state = States::SeedToSoilData;},
            States::SeedToSoilData => {result.seed_to_soil.add(line)}

            States::SoilToFertilizer => {state = States::SoilToFertilizerData;},
            States::SoilToFertilizerData => {result.soil_to_fertilizer.add(line)}

            States::FertilizerToWater => {state = States::FertilizerToWaterData;},
            States::FertilizerToWaterData => {result.fertilizer_to_water.add(line)}

            States::WaterToLight => {state = States::WaterToLightData;},
            States::WaterToLightData => {result.water_to_light.add(line)}

            States::LightToTemperature => {state = States::LightToTemperatureData;},
            States::LightToTemperatureData => {result.light_to_temperature.add(line)}

            States::TemperatureToHumidity => {state = States::TemperatureToHumidityData;},
            States::TemperatureToHumidityData => {result.temperature_to_humidity.add(line)}

            States::HumidityToLocation => {state = States::HumidityToLocationData;},
            States::HumidityToLocationData => {result.humidity_to_location.add(line)}
            _ => {}
        }
    }

    result.get_min_location_range()
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Day5 - Part1: {}", part1(&input));
    println!("Day5 - Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tast0() {
        let str = "seeds: 11 22 33 44\n";
        let num: Vec<u64> = str
                        .trim()
                        .split(':')
                        .nth(1)
                        .unwrap() 
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
                    assert_eq!(num[0], 11);
                    assert_eq!(num[1], 22);
                    assert_eq!(num[2], 33);
                    assert_eq!(num[3], 44);
    }
    #[test]
    fn test1() {
        let result = part1("seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4");
        assert_eq!(result, 35);
    }

    #[test]
    fn test2() {
        let result = part2("seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4");
        assert_eq!(result, 46);
    }
}