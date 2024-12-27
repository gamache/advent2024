use std::collections::HashSet;

use rayon::prelude::*;
use regex::Regex;

pub fn run(input: &str) {
    let cpu = CPU::from_input(input);
    let mut cpu1 = cpu.clone();
    cpu1.run();
    println!("part 1: {}", cpu1.output_string());

    part2(&cpu);
}

#[derive(Debug, Clone)]
struct Path {
    pub a: u64,
    pub i: usize,
}

fn part2(cpu: &CPU) {
    let mut paths: Vec<Path> = vec![];

    for a in 1..1024 {
        let mut cpu2 = cpu.clone();
        cpu2.a = a;
        cpu2.run();
        if cpu2.output.last().unwrap() == cpu2.program.last().unwrap() {
            paths.push(Path { a, i: 1 });
        }
    }

    let mut solutions: Vec<u64> = paths
        .par_iter()
        .flat_map(|p| {
            let mut ps: Vec<Path> = vec![p.clone()];
            let mut solutions: Vec<u64> = vec![];
            while let Some(p) = ps.pop() {
                for n in 0..8 {
                    let mut cpu2 = cpu.clone();
                    let a = (p.a << 3) + n;
                    cpu2.a = a;
                    cpu2.run();

                    if cpu2.program == cpu2.output {
                        solutions.push(a);
                        continue;
                    }
                    if cpu2.output.len() > cpu2.program.len() {
                        continue;
                    }

                    let output_tail = &cpu2.output[(cpu2.output.len() - p.i - 1)..];
                    let program_tail = &cpu2.program[(cpu2.program.len() - output_tail.len())..];
                    if output_tail == program_tail {
                        ps.push(Path { a, i: p.i + 1 });
                    }
                }
            }
            solutions
        })
        .collect();
    solutions.sort();
    println!("part 2: {}", solutions[0]);
}

#[derive(Debug, Clone)]
struct CPU {
    pub program: Vec<u64>,
    pub ip: usize,
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub output: Vec<u64>,
    pub halt: bool,
}

impl CPU {
    pub fn from_input(input: &str) -> CPU {
        let a_re = Regex::new(r"Register A: (\d+)").unwrap();
        let b_re = Regex::new(r"Register B: (\d+)").unwrap();
        let c_re = Regex::new(r"Register C: (\d+)").unwrap();
        let program_re = Regex::new(r"Program: ([,\d]+)").unwrap();
        let a: u64 = a_re
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let b: u64 = b_re
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let c: u64 = c_re
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let program: Vec<u64> = program_re
            .captures(input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(",")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        CPU {
            program,
            a,
            b,
            c,
            ip: 0,
            output: vec![],
            halt: false,
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo operand"),
        }
    }

    fn fetch(&mut self) -> u64 {
        let value = self.program[self.ip];
        self.ip += 1;
        value
    }

    pub fn run(&mut self) {
        // println!("{:?}", self);
        while !self.halt {
            self.tick();
            // println!("{:?}", self);
        }
    }

    pub fn tick(&mut self) {
        match self.fetch() {
            0 => {
                // adv
                let v = self.fetch();
                self.a = self.a / (1 << self.combo(v));
            }
            1 => {
                // bxl
                let v = self.fetch();
                self.b = self.b ^ v;
            }
            2 => {
                // bst
                let v = self.fetch();
                self.b = self.combo(v) % 8;
            }
            3 => {
                // jnz
                let v = self.fetch();
                if self.a != 0 {
                    self.ip = v as usize;
                }
            }
            4 => {
                // bxc
                let _v = self.fetch();
                self.b = self.b ^ self.c;
            }
            5 => {
                // out
                let v = self.fetch();
                self.output.push(self.combo(v) % 8);
            }
            6 => {
                // bdv
                let v = self.fetch();
                self.b = self.a / (1 << self.combo(v));
            }
            7 => {
                // cdv
                let v = self.fetch();
                self.c = self.a / (1 << self.combo(v));
            }
            x => panic!("bad opcode {}", x),
        }

        if self.ip >= self.program.len() {
            self.halt = true
        }
    }

    pub fn output_string(&self) -> String {
        self.output
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(",")
    }
}
