use std::collections::{HashMap, HashSet};

pub fn run(input: &String) {
    let stones: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    part1(&stones);
    part2(&stones);
}

fn part1(stones: &Vec<usize>) {
    println!("part 1: {}", blink_n(stones, 25).len());
}

fn part2(stones: &Vec<usize>) {
    let mut memo25: HashMap<usize, Vec<usize>> = HashMap::new();

    stones.iter().for_each(|stone| {
        if None == memo25.get(stone) {
            memo25.insert(*stone, blink_n(&vec![*stone], 25));
        }
    });

    memo25.clone().iter().for_each(|(_, v)| {
        let set: HashSet<usize> = HashSet::from_iter(v.clone());
        set.iter().for_each(|stone| {
            if None == memo25.get(stone) {
                memo25.insert(*stone, blink_n(&vec![*stone], 25));
            }
        });
    });

    memo25.clone().iter().for_each(|(_, v)| {
        let set: HashSet<usize> = HashSet::from_iter(v.clone());
        set.iter().for_each(|stone| {
            if None == memo25.get(stone) {
                memo25.insert(*stone, blink_n(&vec![*stone], 25));
            }
        });
    });

    let mut s2memo: HashMap<usize, usize> = HashMap::new();

    let count: usize = stones
        .iter()
        .map(|s1| {
            memo25
                .get(s1)
                .unwrap()
                .iter()
                .map(|s2| match s2memo.get(s2) {
                    Some(c) => *c,
                    None => {
                        let c = memo25
                            .get(s2)
                            .unwrap()
                            .iter()
                            .map(|s3| memo25.get(s3).unwrap().len())
                            .sum::<usize>();
                        s2memo.insert(*s2, c);
                        c
                    }
                })
                .sum::<usize>()
        })
        .sum();

    println!("part 2: {}", count)
}

fn blink_n(start_stones: &Vec<usize>, times: usize) -> Vec<usize> {
    let mut stones = start_stones.clone();
    for _ in 0..times {
        stones = blink(&stones);
    }
    stones
}

fn blink(stones: &Vec<usize>) -> Vec<usize> {
    let mut new_stones: Vec<usize> = vec![];
    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }

        let stone_str = format!("{}", stone);
        if stone_str.len() % 2 == 0 {
            let a = stone_str[0..(stone_str.len() / 2)]
                .parse::<usize>()
                .unwrap();
            let b = stone_str[(stone_str.len() / 2)..stone_str.len()]
                .parse::<usize>()
                .unwrap();
            new_stones.push(a);
            new_stones.push(b);
            continue;
        }

        new_stones.push(stone * 2024);
    }
    new_stones
}
