use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn run(lines: &Vec<String>) {
    let initials: Vec<i64> = lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();

    let secrets2k: Vec<i64> = initials.iter().map(|n| step(*n, 2000)).collect();
    let sum: i64 = secrets2k.iter().sum();
    println!("part 1: {}", sum);

    let prices: Vec<Vec<i64>> = initials
        .iter()
        .map(|n| {
            let mut nn = *n;
            let mut v: Vec<i64> = vec![];
            v.push(nn % 10);
            for _ in 0..2000 {
                nn = step(nn, 1);
                v.push(nn % 10);
            }
            v
        })
        .collect();

    let deltas: Vec<Vec<i64>> = prices
        .iter()
        .map(|vals| {
            let mut v: Vec<i64> = vec![0];
            let mut last = vals[0];
            for i in 1..vals.len() {
                v.push(vals[i] - last);
                last = vals[i];
            }
            v
        })
        .collect();

    let seq_prices: Vec<HashMap<Vec<i64>, i64>> = deltas
        .iter()
        .enumerate()
        .map(|(buyer, ds)| {
            let mut sp: HashMap<Vec<i64>, i64> = HashMap::new();
            for i in 3..ds.len() {
                let seq = vec![ds[i - 3], ds[i - 2], ds[i - 1], ds[i]];
                let price = prices[buyer][i];
                if sp.get(&seq) == None {
                    sp.insert(seq, price);
                }
            }
            sp
        })
        .collect();

    let seqs: HashSet<Vec<i64>> = seq_prices
        .iter()
        .flat_map(|sp| sp.keys().into_iter().map(|k| k.clone()))
        .collect();

    let best_price: i64 = seqs
        .par_iter()
        .map(|seq| {
            let mut price = 0;
            for sp in &seq_prices {
                if let Some(p) = sp.get(seq) {
                    price += *p;
                }
            }
            price
        })
        .max()
        .unwrap();

    println!("part 2: {}", best_price);
}

fn step(n: i64, times: i64) -> i64 {
    let mut next = n;
    for _ in 0..times {
        next = next ^ (next << 6);
        next = next % 16777216;
        next = next ^ (next >> 5);
        next = next % 16777216;
        next = next ^ (next << 11);
        next = next % 16777216;
    }
    next
}
