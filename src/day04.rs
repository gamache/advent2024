use crate::Coord;
use crate::Grid;
use std::collections::HashMap;

pub fn run(lines: &Vec<String>) {
    let nrows = lines.len();
    let mut ncols = 0usize;

    let mut coords: HashMap<Coord, String> = HashMap::new();
    for row in 0..lines.len() {
        let chars: Vec<&str> = lines[row].split("").collect();
        ncols = chars.len();
        for col in 0..ncols {
            coords.insert(Coord::new(row, col), chars[col].to_string());
        }
    }

    let grid = Grid {
        coords,
        nrows,
        ncols,
    };
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    let mut count = 0usize;
    let x = String::from("X");
    let m = String::from("M");
    let a = String::from("A");
    let s = String::from("S");
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            let coord = Coord::new(row, col);
            if grid.coords.get(&coord) == Some(&x) {
                for dir in Coord::directions() {
                    if grid.coords.get(&coord.add(&dir, 1)) == Some(&m)
                        && grid.coords.get(&coord.add(&dir, 2)) == Some(&a)
                        && grid.coords.get(&coord.add(&dir, 3)) == Some(&s)
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("part 1: {}", count);
}

fn part2(grid: &Grid) {
    let mut count = 0usize;
    let m = String::from("M");
    let a = String::from("A");
    let s = String::from("S");

    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            let coord = Coord::new(row, col);
            if grid.coords.get(&coord) == Some(&a) {
                let up_right = coord.add(&Coord { row: -1, col: 1 }, 1);
                let up_left = coord.add(&Coord { row: -1, col: -1 }, 1);
                let down_right = coord.add(&Coord { row: 1, col: 1 }, 1);
                let down_left = coord.add(&Coord { row: 1, col: -1 }, 1);

                if (grid.coords.get(&up_right) == Some(&m)
                    && grid.coords.get(&down_left) == Some(&s))
                    || (grid.coords.get(&up_right) == Some(&s)
                        && grid.coords.get(&down_left) == Some(&m))
                {
                    if (grid.coords.get(&up_left) == Some(&m)
                        && grid.coords.get(&down_right) == Some(&s))
                        || (grid.coords.get(&up_left) == Some(&s)
                            && grid.coords.get(&down_right) == Some(&m))
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("part 2: {}", count);
}
