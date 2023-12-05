use std::str::Lines;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

#[derive(Debug)]
struct MapEntry {
    destination: u64,
    source: u64,
    range_length: u64,
}
impl MapEntry {
    fn new(destination: u64, source: u64, range_length: u64) -> Self {
        Self {
            destination,
            source,
            range_length,
        }
    }
    fn get(&self, value: u64) -> Option<u64> {
        if value >= self.source && (self.source..self.source + self.range_length).contains(&value) {
            Some(value - self.source + self.destination)
        } else {
            None
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut seeds = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    input.split("\n\n").for_each(|part| {
        let mut lines = part.lines();
        let mut split = lines.next().unwrap().split(':');
        let key = split.next().unwrap();
        match key {
            "seeds" => split
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .for_each(|s| {
                    seeds.push(s.parse::<u64>().unwrap());
                }),
            "seed-to-soil map" => process_map(lines, &mut seed_to_soil),
            "soil-to-fertilizer map" => process_map(lines, &mut soil_to_fertilizer),
            "fertilizer-to-water map" => process_map(lines, &mut fertilizer_to_water),
            "water-to-light map" => process_map(lines, &mut water_to_light),
            "light-to-temperature map" => process_map(lines, &mut light_to_temperature),
            "temperature-to-humidity map" => process_map(lines, &mut temperature_to_humidity),
            "humidity-to-location map" => process_map(lines, &mut humidity_to_location),
            key => unreachable!("Invalid key: {}", key),
        }
    });

    let maps = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    seeds
        .into_iter()
        .map(|s| find_location(s, &maps))
        .min()
        .unwrap()
}

fn process_map(lines: Lines, map: &mut Vec<MapEntry>) {
    lines
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
            MapEntry::new(
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
                parts[2].parse::<u64>().unwrap(),
            )
        })
        .for_each(|entry| {
            map.push(entry);
        })
}

fn find_location(seed: u64, maps: &[Vec<MapEntry>; 7]) -> u64 {
    let mut current = seed;
    for map in maps.iter() {
        current = map
            .iter()
            .find_map(|entry| entry.get(current))
            .unwrap_or(current);
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }
}
