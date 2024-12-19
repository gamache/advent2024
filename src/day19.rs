use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
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

    let solutions: Vec<Vec<String>> = desired
        .iter()
        .flat_map(|&d| solve(&String::from(d), &available))
        .collect();

    println!("part 1: {}", solutions.len());
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

fn solve(desired: &str, available: &Trie<u8>) -> Option<Vec<String>> {
    let start = Solution {
        prefix: vec![],
        suffix: String::from(desired),
        len: 0,
    };
    let mut heap = BinaryHeap::new();
    heap.push(start);
    let mut visited: HashSet<String> = HashSet::new();

    while let Some(s) = heap.pop() {
        // println!("{:?}", s);
        let joined_prefix = s.prefix.join("");
        if joined_prefix == desired {
            return Some(s.prefix);
        }
        if visited.contains(&joined_prefix) || joined_prefix.len() > desired.len() {
            continue;
        }
        visited.insert(joined_prefix.clone());

        let prefixes: Vec<String> = available.common_prefix_search(&s.suffix).collect();
        for prefix in prefixes {
            let mut next_prefix = s.prefix.clone();
            next_prefix.push(prefix.clone());
            let suffix = String::from(&s.suffix[prefix.len()..]);
            let len: usize = next_prefix.iter().map(|p| p.len()).sum();
            heap.push(Solution {
                prefix: next_prefix,
                suffix,
                len,
            });
        }
    }

    None
}
