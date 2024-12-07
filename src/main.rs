use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

mod day07;

fn main() {
    day07::run(&read_lines("inputs/day07.txt"));
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}
impl Coord {
    pub fn new(row: usize, col: usize) -> Coord {
        Coord {
            row: row as i32,
            col: col as i32,
        }
    }

    pub fn add(&self, coord: &Coord, times: i32) -> Coord {
        Coord {
            row: self.row + (coord.row * times),
            col: self.col + (coord.col * times),
        }
    }

    pub fn directions() -> Vec<Coord> {
        let mut dirs: Vec<Coord> = vec![];
        for row in [-1, 0, 1] {
            for col in [-1, 0, 1] {
                if !(row == 0 && col == 0) {
                    dirs.push(Coord { row, col });
                }
            }
        }
        dirs
    }
}

pub struct Grid {
    pub coords: HashMap<Coord, String>,
    pub nrows: usize,
    pub ncols: usize,
}
impl Grid {
    pub fn print(&self) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                print!("{}", self.coords[&Coord::new(row, col)]);
            }
            println!("");
        }
    }
    pub fn from_lines(lines: &Vec<String>) -> Grid {
        let nrows = lines.len();
        let mut ncols = 0usize;

        let mut coords: HashMap<Coord, String> = HashMap::new();
        for row in 0..lines.len() {
            // chars will have "" as its first and last elements -- ignore
            let chars: Vec<&str> = lines[row].split("").collect();
            ncols = chars.len() - 2;
            for col in 0..ncols {
                coords.insert(Coord::new(row, col), chars[col + 1].to_string());
            }
        }

        Grid {
            coords,
            nrows,
            ncols,
        }
    }

    pub fn find(&self, str: &str) -> Option<Coord> {
        let string = String::from(str);
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let coord = Coord::new(row, col);
                match self.coords.get(&coord) {
                    Some(s) => {
                        if s == &string {
                            return Some(coord);
                        }
                    }
                    _ => (),
                }
            }
        }
        None
    }
}
