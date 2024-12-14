use std::collections::HashMap;

use crate::Coord;
use crate::Grid;
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
    /*
        let width = 101;
        let height = 103;
        let mut moved_robots = robots.clone();

        for i in 1..10001 {
            let mut grid = Grid {
                coords: HashMap::new(),
                nrows: height as usize,
                ncols: width as usize,
            };
            moved_robots = moved_robots
                .iter()
                .map(|robot| robot.go(1, width, height))
                .collect();
            for r in &moved_robots {
                grid.coords.insert(
                    Coord {
                        row: r.y as i32,
                        col: r.x as i32,
                    },
                    String::from("X"),
                );
            }
            println!("\nafter {} seconds:", i);
            grid.print();
        }


        Look through the above output, and notice two things:
        * there's vertical alignment at t=12, and every 103 afterwards
        * there's horizontal alignment at t=35, and every 101 afterwards
        So t mod 103 = 12, and t mod 101 = 35. Chinese Remainder Theorem
        would work, but a brute force search is easier.
    */

    let mut a = 12;
    let mut b = 35;
    loop {
        if a < b {
            a += 103;
        }
        if a == b {
            break;
        }
        if a > b {
            b += 101;
        }
        if a == b {
            break;
        }
    }
    println!("part 2: {}", a);
}
