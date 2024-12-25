use itertools::Itertools;
use std::collections::HashMap;

use crate::Coord;
pub fn run(lines: &Vec<String>) {
    println!("part 1: {}", solve(lines, 3));
    println!("part 2: {}", solve(lines, 26));
}

fn solve(lines: &Vec<String>, robots_with_dpads: usize) -> usize {
    let mut memo: HashMap<(char, char, usize), usize> = HashMap::new();
    lines
        .iter()
        .map(|line| {
            let len = len_from(line, robots_with_dpads, &mut keypad_coords(), &mut memo);
            let num = line[0..(line.len() - 1)].parse::<usize>().unwrap();
            len * num
        })
        .sum()
}

fn len_from(
    line: &str,
    robots_with_dpads: usize,
    coords: &HashMap<char, Coord>,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if robots_with_dpads == 0 {
        return line.len();
    }

    let mut from = 'A';
    let mut len = 0;
    for to in line.chars() {
        match memo.get(&(from, to, robots_with_dpads)) {
            Some(l) => len += l,
            None => {
                let moves = get_moves(coords, from, to);
                let mut min: Option<usize> = None;
                for m in moves {
                    let mut mm = m.clone();
                    mm.push('A');
                    let mstr: String = mm.iter().collect();
                    let len = len_from(&mstr, robots_with_dpads - 1, &dpad_coords(), memo);
                    match min {
                        Some(l) if l <= len => (),
                        _ => min = Some(len),
                    }
                }
                memo.insert((from, to, robots_with_dpads), min.unwrap());
                len += min.unwrap();
            }
        };
        from = to;
    }
    len
}

fn keypad_coords() -> HashMap<char, Coord> {
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
    coords.insert('X', Coord { row: 3, col: 0 });
    coords.insert('0', Coord { row: 3, col: 1 });
    coords.insert('A', Coord { row: 3, col: 2 });
    coords
}

fn dpad_coords() -> HashMap<char, Coord> {
    let mut coords: HashMap<char, Coord> = HashMap::new();
    coords.insert('X', Coord { row: 0, col: 0 });
    coords.insert('^', Coord { row: 0, col: 1 });
    coords.insert('A', Coord { row: 0, col: 2 });
    coords.insert('<', Coord { row: 1, col: 0 });
    coords.insert('v', Coord { row: 1, col: 1 });
    coords.insert('>', Coord { row: 1, col: 2 });
    coords
}

fn get_moves(coords: &HashMap<char, Coord>, from: char, to: char) -> Vec<Vec<char>> {
    let mut moves: Vec<char> = vec![];
    let from_coord = coords[&from];
    let to_coord = coords[&to];

    if to_coord.row > from_coord.row {
        for _ in from_coord.row..to_coord.row {
            moves.push('v');
        }
    }
    if to_coord.col > from_coord.col {
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
        .iter()
        .permutations(moves.len())
        .filter(|perm| {
            let mut coord = from_coord;
            let mut allowed = true;
            for &c in perm {
                match *c {
                    '^' => coord.row -= 1,
                    'v' => coord.row += 1,
                    '<' => coord.col -= 1,
                    '>' => coord.col += 1,
                    _ => panic!("shit"),
                };
                if coords[&'X'] == coord {
                    allowed = false;
                    break;
                }
            }
            allowed
        })
        .map(|v| v.iter().map(|&c| *c).collect::<Vec<char>>())
        .collect()
}
