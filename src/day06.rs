use rayon::prelude::*;
use std::collections::HashSet;

use crate::Coord;
use crate::Grid;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    let part1_visited = part1(&grid);
    part2(&grid, &part1_visited);
}

fn part1(grid: &Grid) -> HashSet<Coord> {
    let mut dir = Coord { row: -1, col: 0 }; // upward
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut coord = grid.find("^").unwrap();
    let empty = String::from(".");
    let start = String::from("^");
    let obstruction = String::from("#");

    loop {
        visited.insert(coord.clone());
        let next_coord = coord.add(&dir, 1);
        match grid.coords.get(&next_coord) {
            Some(s) if s == &empty || s == &start => {
                coord = next_coord;
            }
            Some(s) if s == &obstruction => {
                dir = right(&dir);
            }
            _ => break,
        }
    }

    println!("part 1: {}", visited.len());

    visited
}

fn part2(grid: &Grid, part1_visited: &HashSet<Coord>) {
    let start_coord = grid.find("^").unwrap();
    let empty = String::from(".");
    let start = String::from("^");
    let obstruction = String::from("#");

    let loop_count: usize = part1_visited
        .par_iter()
        .map(|new_obstruction| {
            let mut loops = 0usize;
            let mut coord = start_coord.clone();
            let mut dir = Coord { row: -1, col: 0 }; // upward
            let mut visited: HashSet<(Coord, Coord)> = HashSet::new();

            'find_loop: loop {
                let coord_and_dir = (coord.clone(), dir.clone());
                if visited.contains(&coord_and_dir) {
                    loops += 1;
                    break 'find_loop;
                }
                visited.insert(coord_and_dir);

                let next_coord = coord.add(&dir, 1);

                match grid.coords.get(&next_coord) {
                    Some(s) if s == &obstruction || &next_coord == new_obstruction => {
                        dir = right(&dir);
                    }
                    Some(s) if s == &empty || s == &start => {
                        coord = next_coord;
                    }
                    _ => break 'find_loop,
                }
            }
            loops
        })
        .sum();

    println!("part 2: {}", loop_count);
}

fn right(coord: &Coord) -> Coord {
    match coord {
        Coord { row: -1, col: 0 } => Coord { row: 0, col: 1 },
        Coord { row: 0, col: 1 } => Coord { row: 1, col: 0 },
        Coord { row: 1, col: 0 } => Coord { row: 0, col: -1 },
        Coord { row: 0, col: -1 } => Coord { row: -1, col: 0 },
        _ => panic!("input must be a unit coord"),
    }
}
