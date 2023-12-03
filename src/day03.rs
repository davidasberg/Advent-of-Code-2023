use std::{
    fs,
    ops::{Deref, DerefMut},
};

fn main() {
    let input = fs::read_to_string("input/day03/day03.txt").unwrap();
    let grid = read_input(&input);
    let numbers = get_numbers(&grid);
    let symbols = get_symbols(&grid);
    let numbers_adjacent_to_symbols = get_numbers_adjacent_to_symbols(&numbers, &symbols);
    let part1 = numbers_adjacent_to_symbols.iter().sum::<u32>();

    let gear_ratios = get_gear_ratios(numbers, symbols);
    let part2 = gear_ratios.iter().map(|(a, b)| a * b).sum::<u32>();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_gear_ratios(numbers: Vec<Number>, symbols: Vec<Symbol>) -> Vec<(u32, u32)> {
    // any two numbers that are adjacent to the same symbol are gear ratios
    let mut gear_ratios: Vec<(u32, u32)> = Vec::new();

    for symbol in symbols {
        if symbol.symbol != '*' {
            continue;
        }
        let mut adjacent_numbers: Vec<u32> = Vec::new();
        for number in &numbers {
            if number.indices.iter().any(|(i, j)| {
                DIRECTIONS.iter().any(|(dx, dy)| {
                    if (*i as i32 + dx, *j as i32 + dy)
                        == (symbol.index.0 as i32, symbol.index.1 as i32)
                    {
                        true
                    } else {
                        false
                    }
                })
            }) {
                adjacent_numbers.push(number.num);
            }
        }

        if adjacent_numbers.len() == 2 {
            gear_ratios.push((adjacent_numbers[0], adjacent_numbers[1]));
        }
    }

    gear_ratios
}

fn get_numbers_adjacent_to_symbols(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> Vec<u32> {
    numbers
        .iter()
        .filter_map(|number| {
            // if any of the indices of the number is adjacent to a symbol, return the number
            if number.indices.iter().any(|(i, j)| {
                symbols.iter().any(
                    |Symbol {
                         symbol: _,
                         index: (x, y),
                     }| {
                        DIRECTIONS.iter().any(|(dx, dy)| {
                            if (*x as i32 + dx, *y as i32 + dy) == (*i as i32, *j as i32) {
                                true
                            } else {
                                false
                            }
                        })
                    },
                )
            }) {
                Some(number.num)
            } else {
                None
            }
        })
        .collect::<Vec<u32>>()
}

fn get_symbols(grid: &Grid) -> Vec<Symbol> {
    // every char is a symbol except for numbers and dots
    let mut symbols: Vec<Symbol> = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symbols.push(Symbol {
                    symbol: c,
                    index: (i, j),
                });
            }
        }
    }
    return symbols;
}

fn get_numbers(grid: &Grid) -> Vec<Number> {
    // *467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    let mut numbers: Vec<Number> = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        let mut num = String::new();
        let mut indices: Vec<usize> = Vec::new();
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                num.push(*c);
                indices.push(j);
            }

            if !c.is_digit(10) || j == line.len() - 1 {
                if !num.is_empty() {
                    numbers.push(Number {
                        num: num.parse().unwrap(),
                        indices: indices.iter().map(|&j| (i, j)).collect(),
                    });
                    num.clear();
                    indices.clear();
                }
            }
        }
    }

    return numbers;
}

fn read_input(input: &str) -> Grid {
    let mut grid = Grid(Vec::new());
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

#[derive(Debug)]
struct Number {
    num: u32,
    indices: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    index: (usize, usize),
}

struct Grid(Vec<Vec<char>>);

impl Deref for Grid {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
