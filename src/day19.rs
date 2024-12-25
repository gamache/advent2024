use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use rayon::prelude::*;
use trie_rs::{Trie, TrieBuilder};

pub fn run(input: &str) {
    let (part1, part2) = input.trim().split_once("\n\n").unwrap();
    let desired: Vec<&str> = part2.split("\n").collect();

    let mut avail_builder = TrieBuilder::new();
    for s in part1.split(", ") {
        avail_builder.push(String::from(s));
    }
    let available = avail_builder.build();

    let mut memo: HashMap<String, usize> = HashMap::new();
    let solutions: Vec<usize> = desired
        .iter()
        .map(|&d| solution_count(&String::from(d), &available, &mut memo))
        .filter(|c| *c > 0)
        .collect();

    println!("part 1: {}", solutions.len());
    println!("part 2: {}", solutions.iter().sum::<usize>());
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Solution {
    pub prefix: Vec<String>,
    pub suffix: String,
    pub len: usize,
}
impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.cmp(&other.len)
    }
}
impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solution_count(desired: &str, available: &Trie<u8>, memo: &mut HashMap<String, usize>) -> usize {
    if desired == "" {
        return 1;
    }
    if let Some(c) = memo.get(desired) {
        return *c;
    }

    let prefixes: Vec<String> = available.common_prefix_search(desired).collect();
    let mut count = 0usize;
    for prefix in prefixes {
        let suffix = desired[(prefix.len())..].to_string();
        count += solution_count(&suffix, available, memo);
    }
    memo.insert(desired.to_string(), count);
    count
}
