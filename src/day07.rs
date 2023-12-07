use std::{fs, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day07/day07.txt").expect("file not found");
    let hands = read_input(&input, true);

    // sort hands
    let mut sorted_hands = hands.clone();
    sorted_hands.sort();

    // multiply the rank of each hand by its bid
    // and sum the results
    let part1 = sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum::<u32>();
    println!("Part 1: {}", part1);

    let hands = read_input(&input, false);

    // sort hands
    let mut sorted_hands = hands.clone();
    sorted_hands.sort();
    dbg!(&sorted_hands);

    // multiply the rank of each hand by its bid
    // and sum the results
    let part2 = sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum::<u32>();
    println!("Part 2: {}", part2);
}

fn read_input(input: &str, j_is_jack: bool) -> Vec<Hand> {
    let input = if !j_is_jack {
        input.replace("J", "*")
    } else {
        input.to_string()
    };

    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let cards = split
                .next()
                .unwrap()
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect::<Vec<Card>>();
            let bid = split.next().unwrap().trim().parse::<u32>().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            "*" => Ok(Card::Joker),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();

        let mut counts = sorted_cards
            .iter()
            .filter(|&card| card != &Card::Joker)
            .dedup_with_count()
            .map(|(count, _)| count)
            .collect_vec();

        counts.sort_unstable();
        counts.reverse();

        match counts[..] {
            [] | [_] => HandType::FiveOfAKind,
            [_, 1] => HandType::FourOfAKind,
            [_, 2] => HandType::FullHouse,
            [_, 1, 1] => HandType::ThreeOfAKind,
            [_, 2, 1] => HandType::TwoPair,
            [_, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand"),
        }
    }

    fn compare_high_card(&self, other: &Self) -> std::cmp::Ordering {
        let self_cards = &self.cards;
        let other_cards = &other.cards;

        for i in 0..self_cards.len() {
            let self_card = &self_cards[i];
            let other_card = &other_cards[i];

            if self_card > other_card {
                return std::cmp::Ordering::Greater;
            } else if self_card < other_card {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type == other_type {
            self.compare_high_card(other)
        } else {
            self_type.cmp(&other_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
