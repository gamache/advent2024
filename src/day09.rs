pub fn run(input: &String) {
    let mut fs: Vec<Option<usize>> = vec![];
    let mut chars = input.trim().chars();
    let mut id = 0usize;
    loop {
        if let Some(c) = chars.next() {
            for _ in 0..c.to_string().parse::<usize>().unwrap() {
                fs.push(Some(id));
            }
        } else {
            panic!("out of input!");
        }
        if let Some(c) = chars.next() {
            for _ in 0..c.to_string().parse::<usize>().unwrap() {
                fs.push(None);
            }
        } else {
            break;
        }
        id += 1;
    }

    part1(&mut fs.clone());
    part2(&mut fs, id);
}

fn part1(fs: &mut Vec<Option<usize>>) {
    compact1(fs);
    println!("part 1: {}", checksum(&fs));
}

fn part2(fs: &mut Vec<Option<usize>>, max_id: usize) {
    compact2(fs, max_id);
    println!("part 2: {}", checksum(&fs));
}

fn compact1(fs: &mut Vec<Option<usize>>) {
    let mut first_empty_index = 0usize;
    let mut last_full_index = fs.len() - 1;
    for i in 0..fs.len() {
        match fs[i] {
            Some(_) => (),
            None => {
                first_empty_index = i;
                break;
            }
        }
    }
    for i in 0..fs.len() {
        let ii = fs.len() - 1 - i;
        match fs[ii] {
            None => (),
            Some(_) => {
                last_full_index = ii;
                break;
            }
        }
    }
    if first_empty_index == last_full_index + 1 {
        return;
    }

    let last_full = fs[last_full_index];
    fs[first_empty_index] = last_full;
    fs[last_full_index] = None;
    compact1(fs);
}

fn compact2(fs: &mut Vec<Option<usize>>, current_id: usize) {
    if current_id == 0 {
        return;
    }
    let id_indexes: Vec<usize> = (0..fs.len())
        .filter(|i| fs[*i] == Some(current_id))
        .collect();
    let id_len = id_indexes.len();
    match find_empty(fs, id_len) {
        Some(i) if i < id_indexes[0] => {
            for j in 0..id_len {
                fs[i + j] = fs[id_indexes[0] + j];
                fs[id_indexes[0] + j] = None;
            }
        }
        _ => (),
    }
    compact2(fs, current_id - 1);
}

fn find_empty(fs: &Vec<Option<usize>>, len: usize) -> Option<usize> {
    let mut start: Option<usize> = None;
    let mut count = 0;
    for i in 0..fs.len() {
        match fs[i] {
            Some(_) => {
                count = 0;
                start = None;
            }
            None => {
                if start == None {
                    start = Some(i);
                }
                count += 1;
                if count >= len {
                    return start;
                }
            }
        }
    }
    None
}

fn checksum(fs: &Vec<Option<usize>>) -> usize {
    let mut sum = 0usize;
    for i in 0..fs.len() {
        match fs[i] {
            None => (),
            Some(id) => {
                sum += id * i;
            }
        }
    }
    sum
}
