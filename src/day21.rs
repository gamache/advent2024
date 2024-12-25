use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::Coord;
pub fn run(lines: &Vec<String>) {
    let sum: usize = lines
        .par_iter()
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

    let d2_set: HashSet<String> = d1
        .par_iter()
        .flat_map(|s| directions_to_directions(s))
        .collect();

    let d3_set: HashSet<String> = d2_set
        .par_iter()
        .flat_map(|s| directions_to_directions(s))
        .collect();

    let mut d3: Vec<String> = d3_set.into_iter().collect();
    d3.sort_by(|a, b| a.len().cmp(&b.len()));
    d3[0].clone()
}

fn keypad_to_directions(keypad: &str) -> Vec<String> {
    let mut pos = 'A';
    let mut directions: Vec<Vec<Vec<char>>> = vec![];

    for c in keypad.chars() {
        directions.push(keypad_move(pos, c));
        directions.push(vec![vec!['A']]);
        pos = c;
    }

    let mut exploded_dirs: Vec<Vec<char>> = vec![vec![]];
    for ds in directions {
        let mut xds: Vec<Vec<char>> = vec![];
        for d in ds {
            for xd in &exploded_dirs {
                let mut xdc = xd.clone();
                // println!("{:?}", xdc);
                xdc.extend(d.clone());
                xds.push(xdc);
            }
        }
        exploded_dirs = xds;
    }
    // println!("exploded_dirs {:?}", exploded_dirs);

    exploded_dirs.iter().map(|xd| xd.iter().collect()).collect()
}

fn directions_to_directions(dir: &str) -> Vec<String> {
    let mut pos = 'A';
    let mut directions: Vec<Vec<Vec<char>>> = vec![];

    for c in dir.chars() {
        directions.push(dpad_move(pos, c));
        directions.push(vec![vec!['A']]);
        pos = c;
    }
    // println!("directions {:?}", directions);

    let mut exploded_dirs: Vec<Vec<char>> = vec![vec![]];
    for ds in directions {
        let mut xds: Vec<Vec<char>> = vec![];
        for d in ds {
            for xd in &exploded_dirs {
                let mut xdc = xd.clone();
                // println!("{:?}", xdc);
                xdc.extend(d.clone());
                xds.push(xdc);
            }
        }
        exploded_dirs = xds;
    }
    // println!("exploded_dirs {:?}", exploded_dirs);

    exploded_dirs.iter().map(|xd| xd.iter().collect()).collect()
}

fn keypad_move(from: char, to: char) -> Vec<Vec<char>> {
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
        for _ in to_coord.row..from_coord.row {
            moves.push('^');
        }
    }
    if to_coord.col > from_coord.col {
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
                if coord.row == 3 && coord.col == 0 {
                    allowed = false;
                    break;
                }
            }
            allowed
        })
        .map(|v| v.iter().map(|&c| *c).collect::<Vec<char>>())
        .collect()
}

fn dpad_move(from: char, to: char) -> Vec<Vec<char>> {
    match (from, to) {
        ('A', '^') => vec![vec!['<']],
        ('A', 'v') => vec![vec!['<', 'v'], vec!['v', '<']],
        ('A', '<') => vec![vec!['v', '<', '<']],
        ('A', '>') => vec![vec!['v']],

        ('^', 'A') => vec![vec!['>']],
        ('^', 'v') => vec![vec!['v']],
        ('^', '<') => vec![vec!['v', '<']],
        ('^', '>') => vec![vec!['v', '>'], vec!['>', 'v']],

        ('v', 'A') => vec![vec!['^', '>'], vec!['>', '^']],
        ('v', '^') => vec![vec!['^']],
        ('v', '<') => vec![vec!['<']],
        ('v', '>') => vec![vec!['>']],

        ('<', 'A') => vec![vec!['>', '>', '^']],
        ('<', '^') => vec![vec!['>', '^']],
        ('<', 'v') => vec![vec!['>']],
        ('<', '>') => vec![vec!['>', '>']],

        ('>', 'A') => vec![vec!['^']],
        ('>', '^') => vec![vec!['<', '^'], vec!['^', '<']],
        ('>', 'v') => vec![vec!['<']],
        ('>', '<') => vec![vec!['<', '<']],

        (c1, c2) if c1 == c2 => vec![vec![]],
        x => panic!("{:?}", x),
    }
}

fn dpad_move_old(from: char, to: char) -> Vec<char> {
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
