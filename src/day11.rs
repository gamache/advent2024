pub fn run(input: &String) {
    let stones: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    part1(&stones);
    part2(&stones);
}

fn part1(start_stones: &Vec<usize>) {
    let mut stones = start_stones.clone();
    for _ in 0..25 {
        stones = blink(&stones);
    }
    println!("part 1: {}", stones.len());
}

fn part2(start_stones: &Vec<usize>) {
    let mut stones = start_stones.clone();
    for _ in 0..75 {
        stones = blink(&stones);
    }
    println!("part 2: {}", stones.len());
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
    //println!("{:?}", new_stones);
    new_stones
}
