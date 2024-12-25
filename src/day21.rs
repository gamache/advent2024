use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::Coord;
pub fn run(lines: &Vec<String>) {
    let mut memo: HashMap<(char, char), String> = HashMap::new();
    // let dpad_chars = ['A', '^', 'v', '<', '>'];
    // for from in &dpad_chars {
    //     for to in &dpad_chars {
    //         let moves = dpad_moves(*from, *to);
    //         let mut dirs: Vec<String> = moves
    //             .into_iter()
    //             .flat_map(|m| {
    //                 let mut s: String = m.iter().collect();
    //                 if &s == "" {
    //                     s = String::from("A");
    //                 }
    //                 let d = directions_to_directions(&s);
    //                 println!("{} {} {} {:?}", from, to, s, d);
    //                 d
    //             })
    //             .collect();
    //         dirs.sort_by(|a, b| a.len().cmp(&b.len()));
    //         memo.insert((*from, *to), dirs[0].clone());
    //     }
    // }
    // println!("memo {:?}", memo);

    //let dirs = keypad_to_directions("029A", &'A');
    // println!("{:?}", dirs);
    // println!("{}", shortest_directions("0", 2, &'A'));
    // println!("{}", shortest_directions("2", 2, &'0'));
    // println!("{}", shortest_directions("9", 2, &'2'));
    // println!("{}", shortest_directions("A", 2, &'9'));
    // println!("{}", shortest_directions("029A", 2, &'A'));
    println!("part 1: {}", solve(lines, 2, &memo));
    // println!("part 2: {}", solve(lines, 25));
}

fn solve(
    lines: &Vec<String>,
    robots_with_dpads: usize,
    memo: &HashMap<(char, char), String>,
) -> usize {
    lines
        .iter()
        .map(|line| {
            let dirs = shortest_directions(line, robots_with_dpads, &'A', memo);
            // println!("{}: {}", line, dirs);
            let num = line[0..(line.len() - 1)].parse::<usize>().unwrap();
            dirs.len() * num
        })
        .sum()
}

fn shortest_directions(
    keypad: &str,
    robots_with_dpads: usize,
    from: &char,
    memo: &HashMap<(char, char), String>,
) -> String {
    let mut dirs = keypad_to_directions(keypad, from);
    // println!("{:?}", dirs);

    for _ in 0..robots_with_dpads {
        let d_set: HashSet<String> = dirs
            .par_iter()
            .flat_map(|s| directions_to_directions(s))
            //.map(|s| directions_to_directions_memo(s, &memo))
            .collect();
        println!("{:?}", d_set);
        dirs = d_set.into_iter().collect();
        let min_len = dirs.iter().map(|d| d.len()).min().unwrap();
        dirs = dirs.into_iter().filter(|d| d.len() == min_len).collect();
    }

    dirs.sort_by(|a, b| a.len().cmp(&b.len()));
    dirs[0].clone()
}

fn directions_to_directions_memo(dirs: &str, memo: &HashMap<(char, char), String>) -> String {
    let mut pos = 'A';
    let mut directions: Vec<String> = vec![];

    for c in dirs.chars() {
        directions.push(memo.get(&(pos, c)).unwrap().clone());
        pos = c;
    }

    directions.join("")
}

fn keypad_to_directions(keypad: &str, from: &char) -> Vec<String> {
    let mut pos = *from;
    let mut directions: Vec<Vec<Vec<char>>> = vec![];

    for c in keypad.chars() {
        let mut moves = keypad_moves(pos, c);
        for i in 0..moves.len() {
            moves[i].push('A');
        }
        directions.push(moves);
        pos = c;
    }
    // println!("{:?}", directions);

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
        let mut moves = dpad_moves(pos, c);
        for i in 0..moves.len() {
            moves[i].push('A');
        }
        directions.push(moves);
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

fn keypad_moves(from: char, to: char) -> Vec<Vec<char>> {
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
    get_moves(&coords, from, to, Coord { row: 3, col: 0 })
}

fn dpad_moves(from: char, to: char) -> Vec<Vec<char>> {
    let mut coords: HashMap<char, Coord> = HashMap::new();
    coords.insert('^', Coord { row: 0, col: 1 });
    coords.insert('A', Coord { row: 0, col: 2 });
    coords.insert('<', Coord { row: 1, col: 0 });
    coords.insert('v', Coord { row: 1, col: 1 });
    coords.insert('>', Coord { row: 1, col: 2 });
    get_moves(&coords, from, to, Coord { row: 0, col: 0 })
}

fn get_moves(coords: &HashMap<char, Coord>, from: char, to: char, bad: Coord) -> Vec<Vec<char>> {
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
                if coord == bad {
                    allowed = false;
                    break;
                }
            }
            allowed
        })
        .map(|v| v.iter().map(|&c| *c).collect::<Vec<char>>())
        .collect()
}
