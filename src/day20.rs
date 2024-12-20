use std::cmp::max;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Cursor;
use std::thread::current;

use rayon::prelude::*;

use crate::Coord;
use crate::Grid;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    // println!("part 1: {}", fast_cheat_path_count(&grid, 2));
    println!("part 2: {}", fast_cheat_path_count(&grid, 20));
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cheat {
    pub start: Coord,
    pub end: Coord,
}
impl Cheat {
    pub fn time(&self) -> usize {
        self.start.distance(&self.end) + 1
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    pub coord: Coord,
    pub prev_coords: Vec<Coord>,
    pub time: usize,
    pub heuristic: usize,
    pub cheat: Option<Cheat>,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.time + other.heuristic).cmp(&(self.time + self.heuristic))
        // other.time.cmp(&self.time)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn fast_cheat_path_count(grid: &Grid, max_cheat_time: usize) -> usize {
    let no_cheat = shortest_path(grid, &None, &None).unwrap();
    let mut paths: Vec<Path> = grid
        .coords
        .par_iter()
        .flat_map(|(coord, s)| {
            if s == &String::from("#") {
                cheats_from(grid, coord, max_cheat_time)
            } else {
                vec![]
            }
        })
        .flat_map(|cheat| {
            let mut g = grid.clone();
            g.coords.insert(cheat.start.clone(), String::from("."));
            shortest_path(
                &g,
                &Some(cheat),
                &Some(no_cheat.time - 100.min(no_cheat.time)),
            )
        })
        .collect();
    paths.len()
}
fn cheats_from(grid: &Grid, start: &Coord, max_time: usize) -> Vec<Cheat> {
    let mut cheats: Vec<Cheat> = vec![];
    for row in (start.row - max_time as i32)..(start.row + max_time as i32 + 1) {
        for col in (start.col - max_time as i32)..(start.col + max_time as i32 + 1) {
            let cheat = Cheat {
                start: start.clone(),
                end: Coord { row, col },
            };
            if cheat.time() > 1 && cheat.time() <= max_time {
                cheats.push(cheat);
            }
        }
    }
    cheats
}

fn shortest_path(grid: &Grid, cheat: &Option<Cheat>, max_time: &Option<usize>) -> Option<Path> {
    let start = grid.find("S").unwrap();
    let end = grid.find("E").unwrap();

    let mut heap = BinaryHeap::new();
    heap.push(Path {
        coord: start,
        prev_coords: vec![],
        time: 0,
        heuristic: start.distance(&end),
        cheat: cheat.clone(),
    });

    let mut visited: HashMap<Coord, usize> = HashMap::new();

    while let Some(path) = heap.pop() {
        if max_time != &None && path.time > max_time.unwrap() {
            return None;
        }

        let mut prev_coords = path.prev_coords.clone();
        prev_coords.push(path.coord.clone());

        if let Some(c) = cheat {
            if c.start == path.coord {
                heap.push(Path {
                    coord: c.end.clone(),
                    prev_coords,
                    time: path.time + c.time(),
                    heuristic: c.end.distance(&end),
                    cheat: path.cheat,
                });
                continue;
            }
        }
        if grid.coords.get(&path.coord) == Some(&String::from("#")) {
            // println!("in a wall");
            continue;
        }
        if path.prev_coords.contains(&path.coord) {
            // println!("already here");
            continue;
        }
        if grid.coords.get(&path.coord) == None {
            // println!("out of bounds");
            continue;
        }

        if path.coord == end {
            println!("solution {:?} {}", path.cheat, path.time);
            return Some(path.clone());
        }

        if let Some(&t) = visited.get(&path.coord) {
            if t < path.time {
                // println!("visited at an earlier time");
                continue;
            }
        }
        visited.insert(path.coord.clone(), path.time);

        heap.push(Path {
            coord: path.coord.up(),
            heuristic: path.coord.up().distance(&end),
            time: path.time + 1,
            prev_coords: prev_coords.clone(),
            cheat: path.cheat.clone(),
        });
        heap.push(Path {
            coord: path.coord.down(),
            heuristic: path.coord.down().distance(&end),
            time: path.time + 1,
            prev_coords: prev_coords.clone(),
            cheat: path.cheat.clone(),
        });
        heap.push(Path {
            coord: path.coord.left(),
            heuristic: path.coord.left().distance(&end),
            time: path.time + 1,
            prev_coords: prev_coords.clone(),
            cheat: path.cheat.clone(),
        });
        heap.push(Path {
            coord: path.coord.right(),
            heuristic: path.coord.right().distance(&end),
            time: path.time + 1,
            prev_coords: prev_coords.clone(),
            cheat: path.cheat.clone(),
        });
    }

    None
}
