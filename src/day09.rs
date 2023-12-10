use std::fs;

fn main() {
    let input = fs::read_to_string("input/day09/day09.txt").expect("File not found!");
    let sequences = read_input(&input);

    let part1 = sequences
        .iter()
        .map(|sequence| sequence.get_next_value())
        .filter(|&x| x != 0)
        .sum::<i64>();
    println!("Part 1: {}", part1);

    let reversed_sequences = sequences
        .iter()
        .map(|sequence| Sequence {
            values: sequence.values.iter().rev().cloned().collect(),
        })
        .collect::<Vec<Sequence>>();

    let part2 = reversed_sequences
        .iter()
        .map(|sequence| sequence.get_next_value())
        .sum::<i64>();
    println!("Part 2: {}", part2);
}

fn read_input(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .collect();
            Sequence { values }
        })
        .collect()
}

#[derive(Debug)]
struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    /// Returns the differenes between each pair of values in the sequence, as a sequence
    /// [1, 2, 3, 4] -> [1, 1, 1]
    /// [0, 3, 6, 9, 12, 15] -> [3, 3, 3, 3, 3]
    /// [3, 3, 3, 3, 3] -> [0, 0, 0, 0]
    fn get_differences(&self) -> Sequence {
        let mut differences = Vec::new();
        for i in 0..self.values.len() - 1 {
            differences.push(self.values[i + 1] - self.values[i]);
        }
        Sequence {
            values: differences,
        }
    }

    /// Returns the next number in the sequence
    fn get_next_value(&self) -> i64 {
        // if all values are the same
        if self.values.iter().all(|&x| x == self.values[0]) {
            return self.values[0];
        }

        // get the differences between each pair of values
        let diff = self.get_differences();
        // find the next value in the sequence
        let next_diff_value = diff.get_next_value();
        // return the last value in the sequence + the next difference
        self.values[self.values.len() - 1] + next_diff_value
    }
}
