use rayon::prelude::*;
use regex::Regex;

pub fn run(input: &String) {
    let a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let cost1: i64 = input
        .trim()
        .split("\n\n")
        .par_bridge()
        .map(|s| {
            let a_caps = a_re.captures(s).unwrap();
            let b_caps = b_re.captures(s).unwrap();
            let prize_caps = prize_re.captures(s).unwrap();
            let press = minimum_presses_closed_form(
                a_caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                a_caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                b_caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                b_caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                prize_caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                prize_caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                Some(100),
            );
            match press {
                Some(p) => cost(&p),
                _ => 0,
            }
        })
        .sum();

    println!("part 1: {}", cost1);

    let cost2: i64 = input
        .trim()
        .split("\n\n")
        .par_bridge()
        .map(|s| {
            let a_caps = a_re.captures(s).unwrap();
            let b_caps = b_re.captures(s).unwrap();
            let prize_caps = prize_re.captures(s).unwrap();
            let press = minimum_presses_closed_form(
                a_caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                a_caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                b_caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                b_caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                prize_caps.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000,
                prize_caps.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000,
                None,
            );
            match press {
                Some(p) => cost(&p),
                _ => 0,
            }
        })
        .sum();

    println!("part 2: {}", cost2);
}

fn minimum_presses_closed_form(
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    x: i64,
    y: i64,
    max: Option<i64>,
) -> Option<(i64, i64)> {
    if (bx * ay - by * ax) == 0 {
        return None;
    }

    let b = (x * ay - y * ax) / (bx * ay - by * ax);
    let a = (x - b * bx) / ax;

    match max {
        Some(m) if a > m || b > m => return None,
        _ => (),
    }

    if a > 0 && b > 0 && (a * ax + b * bx) == x && (a * ay + b * by) == y {
        return Some((a, b));
    }

    None
}

fn cost(press: &(i64, i64)) -> i64 {
    press.0 * 3 + press.1
}
