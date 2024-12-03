use regex::Regex;

pub fn run(input: &String) {
    part1(input);
    part2(input);
}

fn part1(input: &String) {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0i32;

    for caps in mul_re.captures_iter(input) {
        let a = caps[1].parse::<i32>().unwrap();
        let b = caps[2].parse::<i32>().unwrap();
        sum += a * b;
    }

    println!("part 1: {}", sum);
}

fn part2(input: &String) {
    let top_re = Regex::new(r"(don't\(\)|do\(\)|mul\(\d+,\d+\))").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0i32;
    let mut ignore = false;

    for caps in top_re.captures_iter(input) {
        match &caps[1] {
            "don't()" => ignore = true,
            "do()" => ignore = false,
            mul => {
                if !ignore {
                    let mul_caps = mul_re.captures(mul).unwrap();
                    let a = mul_caps[1].parse::<i32>().unwrap();
                    let b = mul_caps[2].parse::<i32>().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    println!("part 2: {}", sum);
}
