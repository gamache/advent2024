use rayon::prelude::*;

pub fn run(lines: &Vec<String>) {
    let equations: Vec<(u64, Vec<u64>)> = lines
        .iter()
        .map(|line| {
            let (result_str, operands_str) = line.trim().split_once(": ").unwrap();
            let result = result_str.parse::<u64>().unwrap();
            let operands = operands_str
                .trim()
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            (result, operands)
        })
        .collect();
    part1(&equations);
    part2(&equations);
}

fn part1(equations: &Vec<(u64, Vec<u64>)>) {
    let total: u64 = equations
        .par_iter()
        .map(|(result, operands)| {
            if could_be_true(*result, operands, false) {
                *result
            } else {
                0
            }
        })
        .sum();
    println!("part 1: {}", total);
}

fn part2(equations: &Vec<(u64, Vec<u64>)>) {
    let total: u64 = equations
        .par_iter()
        .map(|(result, operands)| {
            if could_be_true(*result, operands, true) {
                *result
            } else {
                0
            }
        })
        .sum();
    println!("part 2: {}", total);
}

fn could_be_true(result: u64, operands: &Vec<u64>, allow_concat: bool) -> bool {
    if operands.len() == 1 {
        return result == operands[0];
    }
    if operands[0] > result {
        return false;
    }

    let mut added: Vec<u64> = vec![];
    let mut multiplied: Vec<u64> = vec![];
    let mut catted: Vec<u64> = vec![];
    for i in 1..operands.len() {
        added.push(operands[i]);
        multiplied.push(operands[i]);
        catted.push(operands[i]);
    }
    added[0] = added[0] + operands[0];
    multiplied[0] = multiplied[0] * operands[0];
    catted[0] = format!("{}{}", operands[0], catted[0])
        .parse::<u64>()
        .unwrap();

    could_be_true(result, &added, allow_concat)
        || could_be_true(result, &multiplied, allow_concat)
        || (allow_concat && could_be_true(result, &catted, allow_concat))
}
