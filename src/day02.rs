use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = fs::read_to_string("input/day02/day02.txt").expect("Error reading file");
    let games = read_input(&input);
    dbg!(&games);

    let part1 = games
        .iter()
        .filter(|g| g.red <= MAX_RED && g.green <= MAX_GREEN && g.blue <= MAX_BLUE)
        .inspect(|g| println!("{:?}", g))
        .map(|g| g.id)
        .sum::<u32>();

    let part2 = games.iter().map(|g| g.red * g.green * g.blue).sum::<u32>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn read_input(input: &str) -> Vec<Game> {
    // map line to game
    // example line:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut game = Game {
                id: 0,
                red: 0,
                green: 0,
                blue: 0,
            };
            let mut parts = line.split(":");
            let id = parts
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            game.id = id;
            let colors = parts.next().unwrap();
            colors.split(";").for_each(|s| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                s.split(",").for_each(|s| {
                    let mut color = s.split_whitespace();
                    let count = color.next().unwrap().parse::<u32>().unwrap();
                    let color = color.next().unwrap();
                    match color {
                        "red" => red += count,
                        "green" => green += count,
                        "blue" => blue += count,
                        _ => panic!("Unknown color"),
                    }
                });
                game.red = red.max(game.red);
                game.green = green.max(game.green);
                game.blue = blue.max(game.blue);
            });
            game
        })
        .collect()
}
