use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::Coord;
use crate::Grid;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Path {
    pub coord: Coord,
    pub direction: Coord,
    pub cost: usize,
    pub prev_coords: Vec<Coord>,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // reverse order
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);
    let start = grid.find("S").unwrap();
    let mut visited: HashMap<(Coord, Coord), usize> = HashMap::new();
    let mut win_paths: Vec<Path> = vec![];
    let mut heap = BinaryHeap::new();
    heap.push(Path {
        coord: start,
        direction: Coord { row: 0, col: 1 },
        cost: 0,
        prev_coords: vec![],
    });

    while let Some(path) = heap.pop() {
        if let Some(win_path) = win_paths.get(0) {
            if path.cost > win_path.cost {
                break;
            }
        }
        //println!("{:?}", path);
        match grid.coords.get(&path.coord) {
            None => continue,
            Some(s) if s == &String::from("#") => continue,
            Some(s) if s == &String::from("E") => win_paths.push(path),
            _ => {
                let p = path.clone();
                let pair = (p.coord, p.direction);
                match visited.get(&pair) {
                    Some(cost) if *cost < path.cost => continue,
                    _ => {
                        visited.insert(pair, path.cost);
                    }
                };

                let mut prev_coords = path.prev_coords.clone();
                prev_coords.push(path.coord.clone());

                let proceed = Path {
                    coord: path.coord.add(&path.direction, 1),
                    direction: path.direction.clone(),
                    cost: path.cost + 1,
                    prev_coords: prev_coords.clone(),
                };
                let turn_left = Path {
                    coord: path.coord.clone(),
                    direction: path.direction.turn_left(),
                    cost: path.cost + 1000,
                    prev_coords: prev_coords.clone(),
                };
                let turn_right = Path {
                    coord: path.coord.clone(),
                    direction: path.direction.turn_right(),
                    cost: path.cost + 1000,
                    prev_coords: prev_coords.clone(),
                };
                heap.push(proceed);
                heap.push(turn_left);
                heap.push(turn_right);
            }
        }
    }

    println!("part 1: {}", win_paths[0].cost);

    let mut tileset: HashSet<Coord> = HashSet::new();
    for path in win_paths {
        tileset.insert(path.coord);
        for coord in path.prev_coords {
            tileset.insert(coord);
        }
    }
    println!("part 2: {}", tileset.len());
}
