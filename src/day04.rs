use std::{cmp::max, collections::HashMap, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input/day04/day04.txt").unwrap();
    let cards = read_input(&input);
    let part1 = cards.iter().map(calculate_points).sum::<u32>();
    let part2 = (1..=cards.len() as u32)
        .map(|id| calculate_scratch_cards(&cards, id))
        .sum::<u32>();
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn calculate_scratch_cards(cards: &Vec<ScratchCard>, id: u32) -> u32 {
    // one winning number = gives you the next scratch card
    // two winning numbers = gives you the next two scratch cards
    // ...
    if id > cards.len() as u32 {
        return 0;
    }
    let card = &cards[id as usize - 1];
    let my_winning_numbers = cards[id as usize - 1]
        .my_numbers
        .iter()
        .filter(|my_number| card.winning_numbers.contains(my_number))
        .count();

    let ids = (id + 1..=id + my_winning_numbers as u32).collect::<Vec<u32>>();

    ids.iter()
        .map(|id| calculate_scratch_cards(cards, *id))
        .sum::<u32>()
        + 1
}

fn calculate_points(card: &ScratchCard) -> u32 {
    // one winning number = 1 point
    // two winning numbers = 2 points
    // three winning numbers = 4 points
    // four winning numbers = 8 points
    // ...

    let my_winning_numbers = card
        .my_numbers
        .iter()
        .filter(|my_number| card.winning_numbers.contains(my_number))
        .count();

    if my_winning_numbers == 0 {
        return 0;
    }
    let exp = max(0, my_winning_numbers as i32 - 1);
    2u32.pow(exp as u32)
}

fn read_input(input: &str) -> Vec<ScratchCard> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}
// Card id: {winning_numbers} | {my_numbers}
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.trim();

        let numbers = line.split(":").nth(1).unwrap().split("|");

        let winning_numbers = numbers
            .clone()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let my_numbers = numbers
            .clone()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let card = ScratchCard {
            winning_numbers,
            my_numbers,
        };
        Ok(card)
    }
}
