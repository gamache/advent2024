use std::collections::HashMap;

use crate::Coord;
use crate::Grid;

pub fn run(input: &String) {
    let (grid_str, directions_str) = input.trim().split_once("\n\n").unwrap();
    let grid_lines: Vec<String> = grid_str.split("\n").map(|s| s.to_string()).collect();
    let directions: Vec<Coord> = directions_str
        .trim()
        .chars()
        .flat_map(|c| match c {
            '^' => Some(Coord { row: -1, col: 0 }),
            'v' => Some(Coord { row: 1, col: 0 }),
            '<' => Some(Coord { row: 0, col: -1 }),
            '>' => Some(Coord { row: 0, col: 1 }),
            _ => None,
        })
        .collect();
    let grid = Grid::from_lines(&grid_lines);
    part1(&grid, &directions);
    part2(&grid, &directions);
}

fn part1(g: &Grid, directions: &Vec<Coord>) {
    let mut grid = g.clone();
    let mut coord = grid.find("@").unwrap();
    for dir in directions {
        coord = go1(&mut grid, &coord, dir);
    }
    let sum: i32 = grid
        .coords
        .iter()
        .map(|(c, s)| {
            if s == &String::from("O") {
                c.row * 100 + c.col
            } else {
                0
            }
        })
        .sum();
    println!("part 1: {}", sum);
}

fn go1(grid: &mut Grid, start: &Coord, direction: &Coord) -> Coord {
    let mut coord = start.clone();
    loop {
        coord = coord.add(direction, 1);
        match grid.coords.get(&coord) {
            Some(s) if s == &String::from("O") => continue,
            Some(s) if s == &String::from(".") => break,
            _ => return start.clone(),
        }
    }
    loop {
        if &coord == start {
            grid.coords.insert(coord.clone(), String::from("."));
            return coord.add(direction, 1);
        }

        let prev = coord.add(direction, -1);
        grid.coords
            .insert(coord.clone(), grid.coords[&prev].clone());
        coord = prev;
    }
}

fn part2(g: &Grid, directions: &Vec<Coord>) {
    let mut grid = doublewide(g);
    let mut coord = grid.find("@").unwrap();
    for dir in directions {
        coord = go2(&mut grid, &coord, dir);
    }
    let sum: i32 = grid
        .coords
        .iter()
        .map(|(c, s)| {
            if s == &String::from("[") {
                c.row * 100 + c.col
            } else {
                0
            }
        })
        .sum();
    println!("part 2: {}", sum);
}

fn doublewide(grid: &Grid) -> Grid {
    let mut wide = Grid {
        coords: HashMap::new(),
        nrows: grid.nrows,
        ncols: grid.ncols * 2,
    };
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            let coord = Coord::new(row, col);
            if let Some(s) = grid.coords.get(&coord) {
                if s == &String::from("O") {
                    wide.coords
                        .insert(Coord::new(row, col * 2), String::from("["));
                    wide.coords
                        .insert(Coord::new(row, col * 2 + 1), String::from("]"));
                } else if s == &String::from("@") {
                    wide.coords
                        .insert(Coord::new(row, col * 2), String::from("@"));
                    wide.coords
                        .insert(Coord::new(row, col * 2 + 1), String::from("."));
                } else {
                    wide.coords.insert(Coord::new(row, col * 2), s.clone());
                    wide.coords.insert(Coord::new(row, col * 2 + 1), s.clone());
                }
            }
        }
    }
    wide
}

fn can_box_move(grid: &Grid, leftside: &Coord, direction: &Coord) -> bool {
    match direction {
        &Coord { row: 0, col: -1 } => match grid.coords.get(&leftside.left()) {
            Some(s) if *s == String::from("]") => {
                can_box_move(grid, &leftside.left().left(), direction)
            }
            Some(s) if *s == String::from(".") => true,
            _ => false,
        },
        &Coord { row: 0, col: 1 } => match grid.coords.get(&leftside.right().right()) {
            Some(s) if *s == String::from("[") => {
                can_box_move(grid, &leftside.right().right(), direction)
            }
            Some(s) if *s == String::from(".") => true,
            _ => false,
        },
        _ => {
            let next_left = leftside.add(direction, 1);
            let can_left_move = match grid.coords.get(&next_left) {
                Some(s) if *s == String::from(".") => true,
                Some(s) if *s == String::from("[") => can_box_move(grid, &next_left, direction),
                Some(s) if *s == String::from("]") => {
                    can_box_move(grid, &next_left.left(), direction)
                }
                _ => false,
            };

            let next_right = next_left.right();
            let can_right_move = match grid.coords.get(&next_right) {
                Some(s) if *s == String::from(".") => true,
                Some(s) if *s == String::from("[") => can_box_move(grid, &next_right, direction),
                Some(s) if *s == String::from("]") => {
                    //can_box_move(grid, &next_right.left(), direction)
                    true
                }
                _ => false,
            };

            can_left_move && can_right_move
        }
    }
}

fn move_box(grid: &mut Grid, leftside: &Coord, direction: &Coord) {
    match direction {
        &Coord { row: 0, col: -1 } => {
            if grid.coords.get(&leftside.left()) == Some(&String::from("]")) {
                move_box(grid, &leftside.left().left(), direction);
            }
            grid.coords.insert(leftside.left(), String::from("["));
            grid.coords.insert(leftside.clone(), String::from("]"));
        }
        &Coord { row: 0, col: 1 } => {
            if grid.coords.get(&leftside.right().right()) == Some(&String::from("[")) {
                move_box(grid, &leftside.right().right(), direction);
            }
            grid.coords.insert(leftside.right(), String::from("["));
            grid.coords
                .insert(leftside.right().right(), String::from("]"));
        }
        _ => {
            let next_left = leftside.add(direction, 1);
            match grid.coords.get(&next_left) {
                Some(s) if *s == String::from("[") => move_box(grid, &next_left, direction),
                Some(s) if *s == String::from("]") => move_box(grid, &next_left.left(), direction),
                _ => (),
            };

            let next_right = next_left.right();
            match grid.coords.get(&next_right) {
                Some(s) if *s == String::from("[") => move_box(grid, &next_right, direction),
                Some(s) if *s == String::from("]") => move_box(grid, &next_right.left(), direction),
                _ => (),
            };

            grid.coords.insert(next_left, String::from("["));
            grid.coords.insert(next_right, String::from("]"));

            grid.coords.insert(leftside.clone(), String::from("."));
            grid.coords.insert(leftside.right(), String::from("."));
        }
    }
}

fn go2(grid: &mut Grid, start: &Coord, direction: &Coord) -> Coord {
    let next = start.add(direction, 1);
    match grid.coords.get(&next) {
        Some(s) if s == &String::from("[") => {
            if can_box_move(grid, &next, direction) {
                move_box(grid, &next, direction);
                grid.coords.insert(next.clone(), String::from("@"));
                grid.coords.insert(start.clone(), String::from("."));
                return next;
            }
        }
        Some(s) if s == &String::from("]") => {
            if can_box_move(grid, &next.left(), direction) {
                move_box(grid, &next.left(), direction);
                grid.coords.insert(next.clone(), String::from("@"));
                grid.coords.insert(start.clone(), String::from("."));
                return next;
            }
        }
        Some(s) if s == &String::from(".") => {
            grid.coords.insert(next.clone(), String::from("@"));
            grid.coords.insert(start.clone(), String::from("."));
            return next;
        }
        _ => (),
    }

    start.clone()
}
