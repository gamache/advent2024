use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn run(lines: &Vec<String>) {
    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in lines {
        let (from, to) = line.split_once("-").unwrap();
        match network.get_mut(from) {
            Some(tos) => {
                tos.insert(to);
            }
            None => {
                let mut tos = HashSet::new();
                tos.insert(to);
                network.insert(from, tos);
            }
        };
        match network.get_mut(to) {
            Some(tos) => {
                tos.insert(from);
            }
            None => {
                let mut tos = HashSet::new();
                tos.insert(from);
                network.insert(to, tos);
            }
        };
    }

    part1(&network);
    part2(&network);
}

fn part1(network: &HashMap<&str, HashSet<&str>>) {
    let t_triples: HashSet<Vec<&str>> = network
        .par_iter()
        .flat_map(|(&c1, tos1)| {
            let mut triples: Vec<Vec<&str>> = vec![];
            for &c2 in tos1 {
                let tos2 = &network[c2];
                for &c3 in tos2 {
                    let tos3 = &network[c3];
                    if tos3.contains(c1) {
                        if c1.starts_with("t") || c2.starts_with("t") || c3.starts_with("t") {
                            let mut triple = vec![c1, c2, c3];
                            triple.sort();
                            triples.push(triple);
                        }
                    }
                }
            }
            triples
        })
        .collect();

    println!("part 1: {}", t_triples.len());
}

fn part2(network: &HashMap<&str, HashSet<&str>>) {
    let max_cliques: HashSet<Vec<String>> = network
        .par_iter()
        .map(|(&start, _next)| max_clique(network, start))
        .collect();

    let mut max = vec![];
    for clique in max_cliques {
        if clique.len() > max.len() {
            max = clique;
        }
    }
    let password = max.join(",");
    println!("part 2: {}", password);
}

fn is_clique(network: &HashMap<&str, HashSet<&str>>, nodes: &Vec<&str>) -> bool {
    for &c1 in nodes {
        for &c2 in nodes {
            if c1 == c2 {
                continue;
            }
            let next = &network[c1];
            if !next.contains(c2) {
                return false;
            }
        }
    }
    true
}

fn max_clique(network: &HashMap<&str, HashSet<&str>>, start: &str) -> Vec<String> {
    let mut max: Vec<&str> = vec![start];

    for &node in network.keys() {
        if max.contains(&node) {
            continue;
        }
        let mut nodes = max.clone();
        nodes.push(node);
        if is_clique(network, &nodes) {
            max = nodes;
        }
    }

    max.sort();
    max.iter().map(|&node| String::from(node)).collect()
}
