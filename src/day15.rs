use crate::Coord;
use crate::Grid;

pub fn run(input: &String) {
    let (grid_str, directions_str) = input.trim().split_once("\n\n").unwrap();
    let grid_lines: Vec<String> = grid_str.split("\n").map(|s| s.to_string()).collect();
    let directions: Vec<Coord> = directions_str
        .trim()
        .chars()
        .flat_map(|c| match c {
            '^' => Some(Coord { row: -1, col: 0 }),
            'v' => Some(Coord { row: 1, col: 0 }),
            '<' => Some(Coord { row: 0, col: -1 }),
            '>' => Some(Coord { row: 0, col: 1 }),
            _ => None,
        })
        .collect();
    let grid = Grid::from_lines(&grid_lines);
    part1(&grid, &directions);
}

fn part1(g: &Grid, directions: &Vec<Coord>) {
    let mut grid = g.clone();
    let mut coord = grid.find("@").unwrap();
    for dir in directions {
        coord = go1(&mut grid, &coord, dir);
    }
    let sum: i32 = grid
        .coords
        .iter()
        .map(|(c, s)| {
            if s == &String::from("O") {
                c.row * 100 + c.col
            } else {
                0
            }
        })
        .sum();
    println!("part 1: {}", sum);
}

fn go1(grid: &mut Grid, start: &Coord, direction: &Coord) -> Coord {
    let mut coord = start.clone();
    loop {
        coord = coord.add(direction, 1);
        match grid.coords.get(&coord) {
            Some(s) if s == &String::from("O") => continue,
            Some(s) if s == &String::from(".") => break,
            _ => return start.clone(),
        }
    }
    loop {
        if &coord == start {
            grid.coords.insert(coord.clone(), String::from("."));
            return coord.add(direction, 1);
        }

        let prev = coord.add(direction, -1);
        grid.coords
            .insert(coord.clone(), grid.coords[&prev].clone());
        coord = prev;
    }
}
