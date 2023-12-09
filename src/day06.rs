use std::{fs, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day06/day06.txt").unwrap();
    let races = read_input(&input);
    let part1 = part1(&races);
    println!("Part1: {}", part1);

    let total_time = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>();

    let total_distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>();

    let part2 = find_wins(&Race {
        time: total_time.unwrap(),
        distance: total_distance.unwrap(),
    });

    println!("Part2: {}", part2);
}

fn part1(races: &Vec<Race>) -> u32 {
    races.iter().map(find_wins).product()
}

fn find_wins(race: &Race) -> u32 {
    let time = race.time;
    let distance_to_beat = race.distance;

    dbg!(time, distance_to_beat);

    let mut wins = 0;
    for i in 0..time {
        let speed = i;
        let time_left = time - i;
        let distance = speed * time_left;
        if distance > distance_to_beat {
            wins += 1;
        }
    }
    dbg!(wins)
}

/// Example input:
/// Time:        53     83     72     88
//  Distance:   333   1635   1289   1532
fn read_input(input: &str) -> Vec<Race> {
    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>();

    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>();

    let races: Vec<Race> = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time: u64::from_str(time).unwrap(),
            distance: u64::from_str(distance).unwrap(),
        })
        .collect();

    races
}

struct Race {
    time: u64,
    distance: u64,
}
