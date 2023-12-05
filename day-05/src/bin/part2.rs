use itertools::Itertools;
use ranges::{GenericRange, Ranges};
use rayon::prelude::*;
use std::{
    ops::{Bound, Range, RangeBounds},
    str::Lines,
};

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
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
    fn source_range(&self) -> Range<u64> {
        self.source..(self.source + self.range_length)
    }
    fn get_offset(&self) -> i64 {
        self.destination as i64 - self.source as i64
    }
}

fn part2(input: &str) -> u64 {
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
                .tuples()
                .for_each(|(s, l)| {
                    let start = s.parse::<u64>().unwrap();
                    let length = l.parse::<u64>().unwrap();
                    seeds.push(start..(start + length));
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
    let seeds = seeds.into_iter().fold(Ranges::new(), |acc, r| acc.union(r));

    find_min_location(seeds, &maps)
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

fn find_min_location(seeds: Ranges<u64>, maps: &[Vec<MapEntry>; 7]) -> u64 {
    let mut new_ranges = seeds;
    for map in maps {
        new_ranges = apply_map(new_ranges, map);
    }

    new_ranges
        .as_slice()
        .par_iter()
        .map(|r| r.into_iter().min().unwrap())
        .min()
        .unwrap()
}

fn apply_map(mut seeds: Ranges<u64>, map: &[MapEntry]) -> Ranges<u64> {
    let mut new_ranges = Ranges::new();
    for entry in map {
        let matching_ranges_for_entry = seeds.clone().intersect(Ranges::from(entry.source_range()));
        seeds = seeds.difference(matching_ranges_for_entry.clone());

        let offset = entry.get_offset();
        let offset_ranges = offset_ranges(matching_ranges_for_entry, offset);

        new_ranges = new_ranges.union(offset_ranges);
    }
    new_ranges.union(seeds)
}

fn offset_ranges(ranges: Ranges<u64>, offset: i64) -> Ranges<u64> {
    ranges
        .as_slice()
        .iter()
        .map(|r| offset_range(*r, offset))
        .collect::<Ranges<u64>>()
}

fn offset_range(range: GenericRange<u64>, offset: i64) -> GenericRange<u64> {
    let range_start = if let Bound::Included(start) = range.start_bound() {
        *start as i128 + offset as i128
    } else {
        panic!("should only be called with included start bound")
    } as u64;
    let range_end = if let Bound::Excluded(end) = range.end_bound() {
        *end as i128 + offset as i128
    } else {
        panic!("should only be called with included start bound")
    } as u64;

    (range_start..range_end).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
