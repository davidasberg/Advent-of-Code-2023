use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
    str::FromStr,
};

use iter_tools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day05/day05.txt").expect("Unable to read file");

    let (seeds, maps) = read_input(&input);

    let seed_ranges = seeds
        .iter()
        .map(|&x| x..(x + 1))
        .collect::<Vec<Range<u64>>>();
    let part1 = find_closest_location(&seed_ranges, &maps);
    println!("Part 1: {}", part1);

    // seed numbers are actually ranges
    // if seeds are: 79 14 55 13
    // then the seeds are actually 79..=92, 55..=67
    // create a new vector of seeds that include all these numbers
    let seed_ranges = seeds
        .iter()
        .tuples()
        .map(|(&x, &y)| x..(x + y))
        .collect::<Vec<Range<u64>>>();

    let part2 = find_closest_location(&seed_ranges, &maps);
    println!("Part 2: {}", part2);
}

// find the closest location that needs a seed
fn find_closest_location(seeds: &Vec<Range<u64>>, maps: &HashMap<(Type, Type), Map>) -> u64 {
    let mut locations = Vec::new();
    for range in seeds.iter() {
        let mut current = vec![range.clone()];
        for map_type in MAP_TYPES.iter() {
            let map = maps.get(map_type).unwrap();
            // look for the map that contains the current value
            // in the source range, and do the mapping
            // if no map is found, then use the same value
            let mapped_ranges = current
                .iter()
                .flat_map(|x| map.translate(x.clone()))
                .collect::<Vec<Range<u64>>>();
            current = mapped_ranges;
        }
        locations.push(current);
    }
    locations.iter().flatten().map(|x| x.start).min().unwrap()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Type {
    Seeds,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

const MAP_TYPES: [(Type, Type); 7] = [
    (Type::Seeds, Type::Soil),
    (Type::Soil, Type::Fertilizer),
    (Type::Fertilizer, Type::Water),
    (Type::Water, Type::Light),
    (Type::Light, Type::Temperature),
    (Type::Temperature, Type::Humidity),
    (Type::Humidity, Type::Location),
];

fn read_input(input: &str) -> (Vec<u64>, HashMap<(Type, Type), Map>) {
    let mut parts = input.split("\n\n").into_iter();
    let seeds = parts
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maps = HashMap::new();
    for map_type in MAP_TYPES.iter() {
        let map = parts
            .next()
            .unwrap()
            .parse::<Map>()
            .expect("Unable to parse map");

        maps.insert(*map_type, map);
    }

    (seeds, maps)
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn translate(&self, range: Range<u64>) -> Vec<Range<u64>> {
        // if we have range = 50..52
        // and we have a map that translates 51..52 to 101..102
        // then we want to return 50..51 and 101..102

        let mut current = range.start;
        let mut ranges = Vec::new();
        for r in self
            .ranges
            .iter()
            .skip_while(|r| r.range.end <= range.start)
        {
            if r.range.start > current {
                ranges.push(current..min(r.range.start, range.end));
                current = r.range.start;
            }
            if current >= range.end {
                break;
            }
            ranges.push(
                (current as i64 + r.shift) as u64
                    ..(min(r.range.end, range.end) as i64 + r.shift) as u64,
            );
            current = r.range.end;
            if current >= range.end {
                break;
            }
        }

        if current < range.end {
            ranges.push(current..range.end);
        }
        ranges
    }
}

#[derive(Debug)]
struct MapRange {
    range: Range<u64>,
    shift: i64,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s
            .lines()
            .skip(1)
            .map(|x| x.parse::<MapRange>().unwrap())
            .collect::<Vec<MapRange>>();

        // sort the ranges by start
        ranges.sort_by(|a, b| a.range.start.cmp(&b.range.start));

        Ok(Map { ranges })
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dest_start, source_start, range_length) = s
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(MapRange {
            range: source_start..(source_start + range_length),
            shift: dest_start as i64 - source_start as i64,
        })
    }
}
