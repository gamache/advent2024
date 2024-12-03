use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

mod day03;

fn main() {
    day03::run(&read_to_string("inputs/day03.txt").unwrap());
}
