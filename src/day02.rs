pub fn run(lines: &Vec<String>) {
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in lines {
        let nums: Vec<i32> = line
            .trim()
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(nums);
    }

    part1(&reports);
    part2(&reports);
}

fn diffs(report: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = vec![];
    let mut report_iter = report.iter();
    let mut prev_level = report_iter.next().unwrap();

    while let Some(level) = report_iter.next() {
        diffs.push(prev_level - level);
        prev_level = level;
    }

    diffs
}

fn part1(reports: &Vec<Vec<i32>>) {
    let mut safe = 0usize;

    'reports: for report in reports {
        let diffs = diffs(report);
        let mut signum = 0i32;

        for diff in diffs {
            if signum == 0 {
                signum = diff.signum();
            }
            if signum != diff.signum() || diff.abs() < 1 || diff.abs() > 3 {
                continue 'reports;
            }
        }
        safe += 1;
    }

    println!("part 1: {}", safe);
}

fn dampened(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = vec![];
    reports.push(report.clone());
    for i in 0..report.len() {
        let mut r = report.clone();
        r.remove(i);
        reports.push(r);
    }
    reports
}

fn part2(reports: &Vec<Vec<i32>>) {
    let mut safe = 0usize;

    'reports: for report in reports {
        let dampened = dampened(report);
        'dampened: for dr in dampened {
            let diffs = diffs(&dr);
            let mut signum = 0i32;

            for diff in diffs {
                if signum == 0 {
                    signum = diff.signum();
                }
                if signum != diff.signum() || diff.abs() < 1 || diff.abs() > 3 {
                    continue 'dampened;
                }
            }
            safe += 1;
            continue 'reports;
        }
    }

    println!("part 2: {}", safe);
}
