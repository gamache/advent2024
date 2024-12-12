use std::collections::HashSet;

use crate::Coord;
use crate::Grid;

pub fn run(lines: &Vec<String>) {
    let grid = Grid::from_lines(lines);

    let mut plots: Vec<Plot> = vec![];
    let mut plotted_coords: HashSet<Coord> = HashSet::new();

    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            let coord = Coord::new(row, col);
            if !plotted_coords.contains(&coord) {
                let plot = Plot::from_coord(&grid, &coord);
                for c in &plot.coords {
                    plotted_coords.insert(c.clone());
                }
                plots.push(plot);
            }
        }
    }

    let cost: usize = plots.iter().map(|plot| plot.area * plot.perimeter).sum();
    println!("part 1: {}", cost);

    let cost2: usize = plots
        .iter()
        .map(|plot| plot.area * plot.count_sides())
        .sum();
    println!("part 2: {}", cost2);
}

struct Plot {
    pub letter: String,
    pub area: usize,
    pub perimeter: usize,
    pub coords: HashSet<Coord>,
}
impl Plot {
    pub fn from_coord(grid: &Grid, coord: &Coord) -> Plot {
        let letter = grid.coords.get(coord).unwrap().clone();
        let mut perimeter = 0usize;
        let mut coords: HashSet<Coord> = HashSet::new();

        let mut next_coords: Vec<Coord> = vec![coord.clone()];
        while next_coords.len() > 0 {
            let c = next_coords.pop().unwrap();
            if coords.contains(&c) {
                continue;
            }
            coords.insert(c.clone());

            let mut c_perimeter = 0usize;

            if grid.coords.get(&c.up()) == Some(&letter) {
                next_coords.push(c.up());
            } else {
                c_perimeter += 1;
            }

            if grid.coords.get(&c.down()) == Some(&letter) {
                next_coords.push(c.down());
            } else {
                c_perimeter += 1;
            }

            if grid.coords.get(&c.left()) == Some(&letter) {
                next_coords.push(c.left());
            } else {
                c_perimeter += 1;
            }

            if grid.coords.get(&c.right()) == Some(&letter) {
                next_coords.push(c.right());
            } else {
                c_perimeter += 1;
            }

            perimeter += c_perimeter;
        }

        Plot {
            letter: letter,
            area: coords.len(),
            perimeter: perimeter,
            coords: coords,
        }
    }

    pub fn count_sides(&self) -> usize {
        /*
          Left and right sides, we detect the topmost segment.
          Top and bottom sides, we detect the leftmost segment.
        */
        let mut sides = 0usize;

        for coord in &self.coords {
            // topmost left
            if self.coords.get(&coord.left()) == None
                && (self.coords.get(&coord.up()) == None
                    || self.coords.get(&coord.up().left()) != None)
            {
                sides += 1;
            }

            // topmost right
            if self.coords.get(&coord.right()) == None
                && (self.coords.get(&coord.up()) == None
                    || self.coords.get(&coord.up().right()) != None)
            {
                sides += 1;
            }

            // leftmost top
            if self.coords.get(&coord.up()) == None
                && (self.coords.get(&coord.left()) == None
                    || self.coords.get(&coord.left().up()) != None)
            {
                sides += 1;
            }

            // leftmost bottom
            if self.coords.get(&coord.down()) == None
                && (self.coords.get(&coord.left()) == None
                    || self.coords.get(&coord.left().down()) != None)
            {
                sides += 1;
            }
        }

        sides
    }
}
