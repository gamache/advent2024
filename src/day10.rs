use std::collections::HashSet;

use crate::Coord;
use crate::Grid;

use rayon::prelude::*;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    let seq: Vec<String> = (0..10).map(|n| n.to_string()).collect();

    let total: usize = grid
        .all_coords()
        .par_iter()
        .map(|coord| score(grid, &vec![coord.clone()], &seq, 0))
        .sum();

    println!("part 1: {}", total);
}

fn part2(grid: &Grid) {
    let seq: Vec<String> = (0..10).map(|n| n.to_string()).collect();

    let total: usize = grid
        .all_coords()
        .par_iter()
        .map(|coord| rating(grid, &vec![coord.clone()], &seq, 0))
        .sum();

    println!("part 2: {}", total);
}

fn score(grid: &Grid, coords: &Vec<Coord>, seq: &Vec<String>, index: usize) -> usize {
    if coords.len() == 0 {
        return 0;
    }
    let mut matching_coords: HashSet<Coord> = HashSet::new();
    for coord in coords
        .iter()
        .filter(|&c| grid.coords.get(c) == Some(&seq[index]))
    {
        matching_coords.insert(coord.clone());
    }

    if index == 9 {
        matching_coords.len()
    } else {
        let mut next_coords: Vec<Coord> = vec![];
        for coord in matching_coords {
            next_coords.push(Coord {
                row: coord.row + 1,
                col: coord.col,
            });
            next_coords.push(Coord {
                row: coord.row - 1,
                col: coord.col,
            });
            next_coords.push(Coord {
                row: coord.row,
                col: coord.col + 1,
            });
            next_coords.push(Coord {
                row: coord.row,
                col: coord.col - 1,
            });
        }
        score(grid, &next_coords, seq, index + 1)
    }
}

fn rating(grid: &Grid, coords: &Vec<Coord>, seq: &Vec<String>, index: usize) -> usize {
    if coords.len() == 0 {
        return 0;
    }
    let mut matching_coords: Vec<Coord> = vec![];
    for coord in coords
        .iter()
        .filter(|&c| grid.coords.get(c) == Some(&seq[index]))
    {
        matching_coords.push(coord.clone());
    }

    if index == 9 {
        matching_coords.len()
    } else {
        let mut next_coords: Vec<Coord> = vec![];
        for coord in matching_coords {
            next_coords.push(Coord {
                row: coord.row + 1,
                col: coord.col,
            });
            next_coords.push(Coord {
                row: coord.row - 1,
                col: coord.col,
            });
            next_coords.push(Coord {
                row: coord.row,
                col: coord.col + 1,
            });
            next_coords.push(Coord {
                row: coord.row,
                col: coord.col - 1,
            });
        }
        rating(grid, &next_coords, seq, index + 1)
    }
}
