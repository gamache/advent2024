use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::Coord;
use crate::Grid;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Path {
    pub coord: Coord,
    pub direction: Coord,
    pub cost: usize,
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
    part1(&grid);
}

fn part1(grid: &Grid) {
    let mut heap = BinaryHeap::new();
    let start = grid.find("S").unwrap();
    heap.push(Path {
        coord: start,
        direction: Coord { row: 0, col: 1 },
        cost: 0,
    });

    let mut visited: HashMap<(Coord, Coord), usize> = HashMap::new();
    let mut win_path: Option<Path> = None;
    while let Some(path) = heap.pop() {
        //println!("{:?}", path);
        match grid.coords.get(&path.coord) {
            None => continue,
            Some(s) if s == &String::from("#") => continue,
            Some(s) if s == &String::from("E") => {
                win_path = Some(path);
                break;
            }
            _ => {
                let p = path.clone();
                let pair = (p.coord, p.direction);
                match visited.get(&pair) {
                    Some(cost) if *cost <= path.cost => continue,
                    _ => {
                        visited.insert(pair, path.cost);
                    }
                };

                let proceed = Path {
                    coord: path.coord.add(&path.direction, 1),
                    direction: path.direction.clone(),
                    cost: path.cost + 1,
                };
                let turn_left = Path {
                    coord: path.coord.clone(),
                    direction: path.direction.turn_left(),
                    cost: path.cost + 1000,
                };
                let turn_right = Path {
                    coord: path.coord.clone(),
                    direction: path.direction.turn_right(),
                    cost: path.cost + 1000,
                };
                heap.push(proceed);
                heap.push(turn_left);
                heap.push(turn_right);
            }
        }
    }

    println!("part 1: {}", win_path.unwrap().cost)
}
