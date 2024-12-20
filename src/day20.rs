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
    part1(&grid);
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    pub coord: Coord,
    pub prev_coords: Vec<Coord>,
    pub time: usize,
    pub heuristic: usize,
    pub cheat: Option<(Coord, Coord)>,
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

fn part1(grid: &Grid) {
    let no_cheat = shortest_path(grid, None).unwrap();

    let paths: Vec<Path> = grid
        .coords
        .par_iter()
        .flat_map(|(coord, s)| {
            if s == &String::from("#") {
                Some(coord)
            } else {
                None
            }
        })
        .flat_map(|coord| {
            let mut g = grid.clone();
            g.coords.insert(coord.clone(), String::from(" "));
            shortest_path(&g, Some(no_cheat.time - 100))
        })
        .collect();

    println!("part 1: {}", paths.len());
}

fn shortest_path(grid: &Grid, max_time: Option<usize>) -> Option<Path> {
    let start = grid.find("S").unwrap();
    let end = grid.find("E").unwrap();

    let mut heap = BinaryHeap::new();
    heap.push(Path {
        coord: start,
        prev_coords: vec![],
        time: 0,
        heuristic: start.distance(&end),
        cheat: Some((Coord::new(0, 0), Coord::new(0, 0))),
    });

    let mut solutions: Vec<Path> = vec![];
    let mut visited: HashMap<(Coord, Option<(Coord, Coord)>), usize> = HashMap::new();
    let mut skip_cheats: HashSet<Option<(Coord, Coord)>> = HashSet::new();

    while let Some(path) = heap.pop() {
        if max_time != None && path.time > max_time.unwrap() {
            return None;
        }
        let mut prev_coords = path.prev_coords.clone();
        prev_coords.push(path.coord.clone());
        // println!(
        //     "heap={} time={} cheat={:?}",
        //     heap.len(),
        //     path.time,
        //     path.cheat
        // );
        // grid.print_path(&prev_coords);

        let in_a_wall = grid.coords.get(&path.coord) == Some(&String::from("#"));

        let lets_cheat = in_a_wall && path.cheat == None;

        if lets_cheat {
            // println!("CHEAT TO WIN");
        }
        if !lets_cheat && in_a_wall {
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
        if let Some(&t) = skip_cheats.get(&path.cheat) {
            continue;
        }
        if path.coord == end {
            println!("solution {:?} {}", path.cheat, path.time);
            solutions.push(path.clone());
            skip_cheats.insert(path.cheat);
            match path.cheat {
                None => break,
                Some(_) => continue,
            }
        }

        if let Some(&t) = visited.get(&(path.coord, path.cheat)) {
            if t < path.time {
                // println!("visited at an earlier time");
                continue;
            }
        }
        visited.insert((path.coord.clone(), path.cheat.clone()), path.time);

        if !lets_cheat {
            heap.push(Path {
                coord: path.coord.up(),
                heuristic: path.coord.up().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: path.cheat,
            });
            heap.push(Path {
                coord: path.coord.down(),
                heuristic: path.coord.down().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: path.cheat,
            });
            heap.push(Path {
                coord: path.coord.left(),
                heuristic: path.coord.left().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: path.cheat,
            });
            heap.push(Path {
                coord: path.coord.right(),
                heuristic: path.coord.right().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: path.cheat,
            });
        }

        if lets_cheat {
            heap.push(Path {
                coord: path.coord.up(),
                heuristic: path.coord.up().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: Some((path.coord.clone(), path.coord.up())),
            });
            heap.push(Path {
                coord: path.coord.down(),
                heuristic: path.coord.down().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: Some((path.coord.clone(), path.coord.down())),
            });
            heap.push(Path {
                coord: path.coord.left(),
                heuristic: path.coord.left().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: Some((path.coord.clone(), path.coord.left())),
            });
            heap.push(Path {
                coord: path.coord.right(),
                heuristic: path.coord.right().distance(&end),
                time: path.time + 1,
                prev_coords: prev_coords.clone(),
                cheat: Some((path.coord.clone(), path.coord.right())),
            });
        }
    }

    if solutions.len() > 0 {
        return Some(solutions[0].clone());
    } else {
        return None;
    }

    /*
    for path in &solutions {
        // grid.print_path(&path.prev_coords);
    }

    let no_cheat_time = solutions.pop().unwrap().time;
    let save_100_count = solutions
        .iter()
        .filter(|s| s.time + 100 <= no_cheat_time)
        .collect::<Vec<_>>()
        .len();
    println!("part 1: {}", save_100_count);
     */
}
