use std::collections::HashMap;

use crate::Coord;
pub fn run(lines: &Vec<String>) {
    let sum: usize = lines
        .iter()
        .map(|line| {
            let dirs = shortest_directions(line);
            println!("{}: {}", line, dirs);
            let num = line[0..(line.len() - 1)].parse::<usize>().unwrap();
            dirs.len() * num
        })
        .sum();
    println!("part 1: {}", sum);
}

fn shortest_directions(keypad: &str) -> String {
    let d1 = keypad_to_directions(keypad);
    let d2 = directions_to_directions(&d1);
    let d3 = directions_to_directions(&d2);
    println!("{}", keypad);
    println!("{}", d1);
    println!("{}", d2);
    println!("{}", d3);
    d3
}

fn keypad_to_directions(keypad: &str) -> String {
    let mut pos = 'A';
    let mut directions: Vec<char> = vec![];

    for c in keypad.chars() {
        directions.extend(keypad_move(pos, c));
        directions.push('A');
        pos = c;
    }
    directions.iter().collect()
}

fn directions_to_directions(dir: &str) -> String {
    let mut pos = 'A';
    let mut directions: Vec<char> = vec![];

    for c in dir.chars() {
        directions.extend(dpad_move(pos, c));
        directions.push('A');
        pos = c;
    }
    directions.iter().collect()
}

fn keypad_move(from: char, to: char) -> Vec<char> {
    let mut coords: HashMap<char, Coord> = HashMap::new();
    coords.insert('7', Coord { row: 0, col: 0 });
    coords.insert('8', Coord { row: 0, col: 1 });
    coords.insert('9', Coord { row: 0, col: 2 });
    coords.insert('4', Coord { row: 1, col: 0 });
    coords.insert('5', Coord { row: 1, col: 1 });
    coords.insert('6', Coord { row: 1, col: 2 });
    coords.insert('1', Coord { row: 2, col: 0 });
    coords.insert('2', Coord { row: 2, col: 1 });
    coords.insert('3', Coord { row: 2, col: 2 });
    coords.insert('0', Coord { row: 3, col: 1 });
    coords.insert('A', Coord { row: 3, col: 2 });

    let mut moves: Vec<char> = vec![];

    let from_coord = coords[&from];
    let to_coord = coords[&to];

    if to_coord.row < from_coord.row {
        // up is always safe
        for _ in to_coord.row..from_coord.row {
            moves.push('^');
        }
    }
    if to_coord.col > from_coord.col {
        // right is always safe
        for _ in from_coord.col..to_coord.col {
            moves.push('>');
        }
    }
    if to_coord.row > from_coord.row {
        for _ in from_coord.row..to_coord.row {
            moves.push('v');
        }
    }
    if to_coord.col < from_coord.col {
        for _ in to_coord.col..from_coord.col {
            moves.push('<');
        }
    }
    moves
}

fn dpad_move(from: char, to: char) -> Vec<char> {
    let mut coords: HashMap<char, Coord> = HashMap::new();
    coords.insert('^', Coord { row: 0, col: 1 });
    coords.insert('A', Coord { row: 0, col: 2 });
    coords.insert('<', Coord { row: 1, col: 0 });
    coords.insert('v', Coord { row: 1, col: 1 });
    coords.insert('>', Coord { row: 1, col: 2 });

    let mut moves: Vec<char> = vec![];

    let from_coord = coords[&from];
    let to_coord = coords[&to];

    if to_coord.row > from_coord.row {
        // down is always safe
        for _ in from_coord.row..to_coord.row {
            moves.push('v');
        }
    }
    if to_coord.col > from_coord.col {
        // right is always safe
        for _ in from_coord.col..to_coord.col {
            moves.push('>');
        }
    }
    if to_coord.row < from_coord.row {
        for _ in to_coord.row..from_coord.row {
            moves.push('^');
        }
    }
    if to_coord.col < from_coord.col {
        for _ in to_coord.col..from_coord.col {
            moves.push('<');
        }
    }
    moves
}
