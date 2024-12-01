pub fn run(lines: &Vec<String>) {
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];
    for line in lines {
        let nums: Vec<&str> = line.split_ascii_whitespace().collect();
        v1.push(nums[0].parse::<i32>().unwrap());
        v2.push(nums[1].parse::<i32>().unwrap());
    }
    part1(&v1, &v2);
    part2(&v1, &v2);
}

fn part1(v1: &Vec<i32>, v2: &Vec<i32>) {
    let mut sorted1 = v1.clone();
    sorted1.sort();
    let mut sorted2 = v2.clone();
    sorted2.sort();

    let mut distance = 0i32;
    for i in 0..sorted1.len() {
        distance += (sorted1[i] - sorted2[i]).abs();
    }

    println!("part 1: {}", distance);
}

fn part2(v1: &Vec<i32>, v2: &Vec<i32>) {
    let mut similarity = 0i32;
    for x in v1 {
        let mut count = 0;
        for y in v2 {
            if x == y {
                count += 1;
            }
        }
        similarity += x * count;
    }

    println!("part 2: {}", similarity);
}
