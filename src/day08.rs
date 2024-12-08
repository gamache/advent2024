use std::collections::HashSet;

use crate::Coord;
use crate::Grid;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    solve(&grid, false);
    solve(&grid, true);
}

fn solve(grid: &Grid, part2: bool) {
    let mut antinodes: HashSet<Coord> = HashSet::new();

    let dot = String::from(".");
    for row in 0..grid.nrows as i32 {
        for col in 0..grid.ncols as i32 {
            let rowcol = Coord { row, col };
            match grid.coords.get(&rowcol) {
                Some(freq) if freq != &dot => {
                    for r in 0..grid.nrows as i32 {
                        for c in 0..grid.ncols as i32 {
                            if row == r && col == c {
                                continue;
                            }

                            let rc = Coord { row: r, col: c };
                            if grid.coords.get(&rc) == Some(freq) {
                                let dir = Coord {
                                    row: row - r,
                                    col: col - c,
                                };

                                if part2 {
                                    let maxtimes = grid.nrows.max(grid.ncols);
                                    for times in 0..maxtimes {
                                        antinodes.insert(rowcol.add(&dir, times as i32));
                                        antinodes.insert(rc.add(&dir, 0 - times as i32));
                                    }
                                } else {
                                    antinodes.insert(rowcol.add(&dir, 1));
                                    antinodes.insert(rc.add(&dir, -1));
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let valid_antinodes: Vec<&Coord> = antinodes
        .iter()
        .filter(|coord| {
            coord.row >= 0
                && coord.row < grid.nrows as i32
                && coord.col >= 0
                && coord.col < grid.ncols as i32
        })
        .collect();
    println!("{}", valid_antinodes.len());
}
