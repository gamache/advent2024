use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::Coord;
use crate::Grid;

pub fn run(input: &Vec<String>) {
    let coords: Vec<Coord> = input
        .iter()
        .map(|s| {
            let (colstr, rowstr) = s.split_once(",").unwrap();
            let row: i32 = rowstr.parse().unwrap();
            let col: i32 = colstr.parse().unwrap();
            Coord { row, col }
        })
        .collect();

    part1(&coords);
    part2(&coords);
}

#[derive(PartialEq, Eq)]
struct Path {
    pub coord: Coord,
    pub len: usize,
    pub prev_coords: Vec<Coord>,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.len.cmp(&self.len) // reverse order
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(coords: &Vec<Coord>) {
    let path = shortest_path(coords, 71, 71, 1024);
    println!("part 1: {}", path.unwrap().len);
}

fn part2(coords: &Vec<Coord>) {
    // binary search
    let mut falls = coords.len() / 2;
    let mut step = falls / 2;
    let mut steps = 0;
    loop {
        if shortest_path(coords, 71, 71, falls) == None {
            falls = falls - step;
        } else {
            falls = falls + step;
        }
        steps += 1;
        step = step / 2;
        if step == 0 {
            break;
        }
    }

    // dirty hack for overcoming step size rounding errors
    for f in falls..(falls + steps) {
        if shortest_path(coords, 71, 71, f) == None {
            println!("part 2: {},{}", coords[f - 1].col, coords[f - 1].row);
            break;
        }
    }
}

fn shortest_path(coords: &Vec<Coord>, nrows: usize, ncols: usize, falls: usize) -> Option<Path> {
    let mut grid = Grid {
        coords: HashMap::new(),
        nrows,
        ncols,
    };
    let start = Coord { row: 0, col: 0 };
    let end = Coord {
        row: nrows as i32 - 1,
        col: ncols as i32 - 1,
    };

    for i in 0..falls {
        grid.coords.insert(coords[i], String::from("#"));
    }

    let mut paths = BinaryHeap::new();
    let mut win_path: Option<Path> = None;
    let start_path = Path {
        coord: start,
        len: 0,
        prev_coords: vec![],
    };
    let mut visited: HashSet<Coord> = HashSet::new();
    paths.push(start_path);
    while let Some(path) = paths.pop() {
        if path.coord == end {
            win_path = Some(path);
            break;
        }
        if !grid.in_bounds(&path.coord)
            || visited.contains(&path.coord)
            || grid.coords.get(&path.coord) == Some(&String::from("#"))
        {
            continue;
        }

        visited.insert(path.coord.clone());
        let mut prev_coords = path.prev_coords.clone();
        prev_coords.push(path.coord.clone());
        paths.push(Path {
            coord: path.coord.up(),
            len: path.len + 1,
            prev_coords: prev_coords.clone(),
        });
        paths.push(Path {
            coord: path.coord.down(),
            len: path.len + 1,
            prev_coords: prev_coords.clone(),
        });
        paths.push(Path {
            coord: path.coord.left(),
            len: path.len + 1,
            prev_coords: prev_coords.clone(),
        });
        paths.push(Path {
            coord: path.coord.right(),
            len: path.len + 1,
            prev_coords: prev_coords.clone(),
        });
    }

    win_path
}
