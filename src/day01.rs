use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/day01.txt").expect("Error reading file");

    let result = input
        .lines()
        .map(find_first_last_digit)
        .map(|s| s.parse::<u32>().unwrap())
        .sum::<u32>();

    let result2 = input
        .lines()
        .map(find_first_last_digit_with_strings)
        .map(|s| s.parse::<u32>().unwrap())
        .sum::<u32>();

    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

fn find_first_last_digit(line: &str) -> String {
    let mut first = 0;
    let mut last = 0;
    let mut first_found = false;
    for c in line.chars() {
        if c.is_digit(10) {
            if !first_found {
                first = c.to_digit(10).unwrap();
                first_found = true;
            }
            last = c.to_digit(10).unwrap();
        }
    }
    format!("{}{}", first, last)
}

fn find_first_last_digit_with_strings(line: &str) -> String {
    let digits: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut first_index = usize::MAX;
    let mut last_index = usize::MIN;
    let mut first_value = "";
    let mut last_value = "";
    for (&key, &value) in digits.iter() {
        if line.find(key) != None && line.find(key).unwrap() <= first_index {
            first_index = line.find(key).unwrap();
            first_value = value;
        }
        if line.rfind(key) != None && line.rfind(key).unwrap() >= last_index {
            last_index = line.rfind(key).unwrap();
            last_value = value;
        }

        if line.find(value) != None && line.find(value).unwrap() <= first_index {
            first_index = line.find(value).unwrap();
            first_value = value;
        }
        if line.rfind(value) != None && line.rfind(value).unwrap() >= last_index {
            last_index = line.rfind(value).unwrap();
            last_value = value;
        }
        println!("{} {}", first_value, last_value);
    }
    println!("{}{}", first_value, last_value);
    format! {"{}{}", first_value, last_value}
}
