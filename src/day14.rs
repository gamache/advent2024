use std::collections::HashMap;

use crate::Coord;
use crate::Grid;
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    pub x: i64,
    pub y: i64,
    pub dx: i64,
    pub dy: i64,
}
impl Robot {
    pub fn go(&self, seconds: i64, width: i64, height: i64) -> Robot {
        let x = (((self.x + seconds * self.dx) % width) + width) % width;
        let y = (((self.y + seconds * self.dy) % height) + height) % height;
        Robot {
            x: x,
            y: y,
            dx: self.dx,
            dy: self.dy,
        }
    }
}

pub fn run(lines: &Vec<String>) {
    let line_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots: Vec<Robot> = lines
        .iter()
        .map(|line| {
            let caps = line_re.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let dx = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let dy = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            Robot { x, y, dx, dy }
        })
        .collect();
    part1(&robots);
    part2(&robots);
}

fn part1(robots: &Vec<Robot>) {
    let width = 101i64;
    let height = 103i64;
    let half_width = width - (width / 2) - 1;
    let half_height = height - (height / 2) - 1;

    let moved_robots: Vec<Robot> = robots
        .iter()
        .map(|robot| robot.go(100, width, height))
        .collect();

    let mut q1 = 0usize;
    let mut q2 = 0usize;
    let mut q3 = 0usize;
    let mut q4 = 0usize;
    for r in moved_robots {
        if r.x > half_width && r.y < half_height {
            q1 += 1;
        }
        if r.x < half_width && r.y < half_height {
            q2 += 1;
        }
        if r.x > half_width && r.y > half_height {
            q3 += 1;
        }
        if r.x < half_width && r.y > half_height {
            q4 += 1;
        }
    }
    println!("part 1: {}", q1 * q2 * q3 * q4);
}

fn part2(robots: &Vec<Robot>) {
    let width = 101;
    let height = 103;
    let mut moved_robots = robots.clone();
    let mut h = 0;
    let mut hmod = 0;
    let mut v = 0;
    let mut vmod = 0;

    // Find instances of horizontal or vertical alignment, and their recurrence periods
    let mut i = 0;
    loop {
        i += 1;

        moved_robots = moved_robots
            .par_iter()
            .map(|robot| robot.go(1, width, height))
            .collect();

        let mut grid = Grid {
            coords: HashMap::new(),
            nrows: height as usize,
            ncols: width as usize,
        };
        for robot in &moved_robots {
            grid.coords.insert(
                Coord {
                    row: robot.y as i32,
                    col: robot.x as i32,
                },
                String::from("X"),
            );
        }

        if has_horiz_alignment(&grid) {
            if h > 0 {
                hmod = i - h;
            } else {
                h = i;
            }
        }
        if has_vert_alignment(&grid) {
            if v > 0 {
                vmod = i - v;
            } else {
                v = i;
            }
        }
        if hmod > 0 && vmod > 0 {
            break;
        }
    }

    // t % hmod == h, and t % vmod == v. Find t.
    loop {
        if h < v {
            h += hmod;
        }
        if h == v {
            break;
        }
        if h > v {
            v += vmod;
        }
        if h == v {
            break;
        }
    }
    println!("part 2: {}", h);
}

// checks if there is a row filled to 20% or more
fn has_horiz_alignment(grid: &Grid) -> bool {
    for row in 0..grid.nrows {
        let mut count = 0usize;
        for col in 0..grid.ncols {
            if grid.coords.get(&Coord::new(row, col)) != None {
                count += 1;
            }
        }
        if count * 5 >= grid.ncols {
            return true;
        }
    }
    false
}

// checks if there is a column filled to 20% or more
fn has_vert_alignment(grid: &Grid) -> bool {
    for col in 0..grid.ncols {
        let mut count = 0usize;
        for row in 0..grid.nrows {
            if grid.coords.get(&Coord::new(row, col)) != None {
                count += 1;
            }
        }
        if count * 5 >= grid.nrows {
            return true;
        }
    }
    false
}
