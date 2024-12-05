use std::cmp::Ordering;

pub fn run(input: &String) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules: Vec<Vec<i32>> = rules_str
        .trim()
        .split("\n")
        .map(|line| line.split("|").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    let updates: Vec<Vec<i32>> = updates_str
        .trim()
        .split("\n")
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    part1(&rules, &updates);
    part2(&rules, &updates);
}

fn part1(rules: &Vec<Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut sum = 0;
    for update in updates {
        if correct_order(update, rules) {
            sum += update[update.len() / 2];
        }
    }
    println!("part 1: {}", sum);
}

fn part2(rules: &Vec<Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut sum = 0;
    for update in updates {
        if !correct_order(update, rules) {
            sum += reorder(update, rules)[update.len() / 2];
        }
    }
    println!("part 2: {}", sum);
}

fn correct_order(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    for i in 0..update.len() {
        let page = update[i];
        for rule in rules {
            let before = rule[0];
            let after = rule[1];
            if page == after {
                match update.iter().position(|x| *x == before) {
                    None => (),
                    Some(index) => {
                        if index > i {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

fn reorder(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut new_update = update.clone();
    new_update.sort_by(|a, b| {
        for rule in rules {
            let before = rule[0];
            let after = rule[1];
            if *a == before && *b == after {
                return Ordering::Less;
            }
            if *b == before && *a == after {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
    new_update
}
