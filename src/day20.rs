use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use rayon::prelude::*;

use crate::Coord;
use crate::Grid;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);

    println!("part 1: {}", solve(&grid, 2));
    println!("part 2: {}", solve(&grid, 20));
    //  872419 too low
    // 1906495 too high
    // 1002528 wrong
    // 1001139 wrong
}

fn solve(grid: &Grid, max_cheat_time: usize) -> usize {
    let start = grid.find(&String::from("S")).unwrap();
    let end = grid.find(&String::from("E")).unwrap();

    let times_from_start = times_from_coord(&grid, &start);
    let times_to_end = times_from_coord(&grid, &end);

    let no_cheat_time = *times_to_end.get(&start).unwrap();
    println!("no cheat {}", no_cheat_time);

    let cheats: HashSet<Cheat> = times_from_start
        .iter()
        .flat_map(|(coord, _)| cheats_from(grid, coord, max_cheat_time))
        .collect();
    println!("{} cheats", cheats.len());

    let mut playable_cheats: HashMap<Coord, HashSet<Cheat>> = HashMap::new();
    for cheat in cheats {
        let mut pcs: HashSet<Cheat> = match playable_cheats.get(&cheat.start) {
            None => HashSet::new(),
            Some(v) => v.clone(),
        };
        pcs.insert(cheat.clone());
        playable_cheats.insert(cheat.start.clone(), pcs);
    }
    // println!("{:?}", playable_cheats);

    let mut cheat_times: HashMap<Cheat, usize> = HashMap::new();
    for (coord, start_time) in times_from_start.iter() {
        if let Some(cheats) = playable_cheats.get(coord) {
            for cheat in cheats {
                // println!("{:?}", cheat);
                if let Some(end_time) = times_to_end.get(&cheat.end) {
                    let time = start_time + cheat.time() + end_time;
                    match cheat_times.get(cheat) {
                        Some(t) if *t < time => None,
                        _ => cheat_times.insert(cheat.clone(), time),
                    };
                }
            }
        }
    }
    // println!("{:?}", cheat_times);
    // grid.print();
    let mut cheat_time_freq: HashMap<usize, usize> = HashMap::new();
    for (cheat, time) in &cheat_times {
        if time + 76 == no_cheat_time {
            println!("{:?}", cheat);
            grid.print_path(&vec![cheat.start, cheat.end]);
        }
        match cheat_time_freq.get(&time) {
            Some(n) => cheat_time_freq.insert(*time, n + 1),
            None => cheat_time_freq.insert(*time, 1),
        };
    }
    println!("{:?}", cheat_time_freq);
    for (time, n) in &cheat_time_freq {
        let time_saved = 84 - (*time as i32);
        if time_saved >= 50 {
            println!("time saved {}, n {}", time_saved, n);
        }
    }

    cheat_times
        .iter()
        .filter(|(_, time)| *time + 100 <= no_cheat_time)
        .collect::<Vec<_>>()
        .len()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Cheat {
    pub start: Coord,
    pub end: Coord,
}
impl Cheat {
    pub fn time(&self) -> usize {
        self.start.distance(&self.end)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    pub coord: Coord,
    pub time: usize,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn times_from_coord(grid: &Grid, coord: &Coord) -> HashMap<Coord, usize> {
    let mut times: HashMap<Coord, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();
    heap.push(Path {
        coord: coord.clone(),
        time: 0,
    });

    while let Some(path) = heap.pop() {
        if grid.coords.get(&path.coord) == Some(&String::from("#")) {
            continue;
        }
        if times.get(&path.coord) == None {
            times.insert(path.coord.clone(), path.time);
        } else {
            continue;
        }
        heap.push(Path {
            coord: path.coord.up(),
            time: path.time + 1,
        });
        heap.push(Path {
            coord: path.coord.down(),
            time: path.time + 1,
        });
        heap.push(Path {
            coord: path.coord.left(),
            time: path.time + 1,
        });
        heap.push(Path {
            coord: path.coord.right(),
            time: path.time + 1,
        });
    }

    times
}

fn cheats_from(grid: &Grid, start: &Coord, max_time: usize) -> Vec<Cheat> {
    let mut cheats: Vec<Cheat> = vec![];
    let t = max_time as i32;
    for row in (start.row - t)..(start.row + t + 1) {
        for col in (start.col - max_time as i32)..(start.col + max_time as i32) {
            let end = Coord { row, col };
            if start == &end {
                continue;
            }
            if grid.coords.get(&end) != Some(&String::from("#")) {
                let cheat = Cheat {
                    start: start.clone(),
                    end,
                };
                if cheat.time() <= t as usize {
                    cheats.push(cheat);
                }
            }
        }
    }
    // println!("{:?}", cheats);
    cheats
}
