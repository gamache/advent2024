use crate::Coord;
use crate::Grid;

pub fn run(input: &String) {
    let grid_strs: Vec<&str> = input.trim().split("\n\n").collect();
    let grids: Vec<Grid> = grid_strs
        .iter()
        .map(|&g| {
            let lines: Vec<String> = g.split("\n").map(String::from).collect();
            Grid::from_lines(&lines)
        })
        .collect();
    let mut locks: Vec<Grid> = vec![];
    let mut keys: Vec<Grid> = vec![];
    for grid in grids {
        if grid.coords.get(&Coord { row: 0, col: 0 }) == Some(&String::from("#")) {
            locks.push(grid);
        } else {
            keys.push(grid);
        }
    }
    let lock_heights: Vec<Vec<usize>> = locks.iter().map(|g| lock_heights(g)).collect();
    let key_heights: Vec<Vec<usize>> = keys.iter().map(|g| key_heights(g)).collect();

    let mut fits = 0;
    for lock in &lock_heights {
        for key in &key_heights {
            if !overlaps(lock, key) {
                fits += 1;
            }
        }
    }
    println!("part 1: {}", fits);
}

fn lock_heights(grid: &Grid) -> Vec<usize> {
    let mut heights = vec![];
    let s = String::from("#");
    for col in 0..grid.ncols {
        let mut height = 0usize;
        for row in 0..grid.nrows {
            if grid.coords.get(&Coord::new(row, col)) == Some(&s) {
                height += 1;
            }
        }
        heights.push(height - 1);
    }
    heights
}

fn key_heights(grid: &Grid) -> Vec<usize> {
    let mut heights = vec![];
    let s = String::from(".");
    for col in 0..grid.ncols {
        let mut height = 0usize;
        for row in 0..grid.nrows {
            if grid.coords.get(&Coord::new(row, col)) == Some(&s) {
                height += 1;
            }
        }
        heights.push(6 - height);
    }
    heights
}

fn overlaps(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    for i in 0..lock.len() {
        if lock[i] + key[i] > 5 {
            return true;
        }
    }
    false
}
