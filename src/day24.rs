use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    pub in1: String,
    pub in2: String,
    pub op: Op,
    pub out: String,
}

pub fn run(input: &str) {
    let (states, insts) = input.trim().split_once("\n\n").unwrap();
    let state_re = Regex::new(r"(.+): (\d)").unwrap();
    let inst_re = Regex::new(r"(\S+) (\S+) (\S+) -> (\S+)").unwrap();

    let mut wires: HashMap<String, u64> = HashMap::new();
    for state in states.split("\n") {
        let caps = state_re.captures(state).unwrap();
        let wire = caps.get(1).unwrap().as_str().to_string();
        let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        wires.insert(wire, value);
    }

    let mut instructions: Vec<Instruction> = vec![];
    for inst in insts.split("\n") {
        let caps = inst_re.captures(inst).unwrap();
        let in1 = caps.get(1).unwrap().as_str().to_string();
        let op = match caps.get(2).unwrap().as_str() {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            x => panic!("bad op {}", x),
        };
        let in2 = caps.get(3).unwrap().as_str().to_string();
        let out = caps.get(4).unwrap().as_str().to_string();
        instructions.push(Instruction { in1, in2, op, out });
    }

    println!("part 1: {}", execute(&wires, &instructions));
    part2(&instructions);
}

fn execute(wires: &HashMap<String, u64>, instructions: &Vec<Instruction>) -> u64 {
    let mut wires = wires.clone();
    loop {
        let mut changed = false;
        for inst in instructions {
            if wires.get(&inst.out) == None {
                if let Some(in1) = wires.get(&inst.in1) {
                    if let Some(in2) = wires.get(&inst.in2) {
                        let out = match inst.op {
                            Op::And => in1 & in2,
                            Op::Or => in1 | in2,
                            Op::Xor => in1 ^ in2,
                        };
                        wires.insert(inst.out.clone(), out);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    number(&wires, "z")
}

fn number(wires: &HashMap<String, u64>, letter: &str) -> u64 {
    let mut zs: Vec<&String> = wires.keys().filter(|&k| k.starts_with(letter)).collect();
    zs.sort();

    let mut result = 0;
    while zs.len() > 0 {
        result = result << 1;
        result = result | wires[&zs.pop().unwrap().to_string()];
    }

    result
}

fn part2(instructions: &Vec<Instruction>) {
    // full adder consists of:
    // (a xor b) xor c0 -> s
    // (a and b) or ((a xor b) and c0) -> c1

    let mut swaps: Vec<String> = vec![];
    let mut carry: HashMap<usize, String> = HashMap::new();
    for i in 0..64 {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        if get_inst1(instructions, &x, &Op::Xor) == None {
            break;
        }
        if i == 0 {
            let aandb = get_inst(instructions, &x, &y, &Op::And).unwrap().out;
            carry.insert(i, aandb);
        } else {
            let mut c0 = carry.get(&(i - 1)).unwrap().clone();
            let mut axorb = get_inst(instructions, &x, &y, &Op::Xor).unwrap().out;
            let mut aandb = get_inst(instructions, &x, &y, &Op::And).unwrap().out;

            let mut s = match get_inst(instructions, &axorb, &c0, &Op::Xor) {
                Some(s) => s.out,
                None => match get_inst1(instructions, &axorb, &Op::Xor) {
                    Some(inst) => {
                        let real_c0 = if inst.in1 == axorb {
                            inst.in2
                        } else {
                            inst.in1
                        };
                        swaps.push(c0.clone());
                        swaps.push(real_c0.clone());
                        c0 = real_c0;
                        inst.out
                    }
                    None => match get_inst1(instructions, &c0, &Op::Xor) {
                        Some(inst) => {
                            let real_axorb = if inst.in1 == c0 { inst.in2 } else { inst.in1 };
                            swaps.push(axorb.clone());
                            swaps.push(real_axorb.clone());
                            axorb = real_axorb;
                            inst.out
                        }
                        None => panic!("shit"),
                    },
                },
            };

            if aandb.starts_with("z") && !s.starts_with("z") {
                let swap = aandb;
                aandb = s;
                s = swap;
                swaps.push(aandb.clone());
                swaps.push(s.clone());
            }
            let mut axorbandc = match get_inst(instructions, &axorb, &c0, &Op::And) {
                Some(s) => s.out,
                None => match get_inst1(instructions, &axorb, &Op::And) {
                    Some(inst) => {
                        let real_c0 = if inst.in1 == axorb {
                            inst.in2
                        } else {
                            inst.in1
                        };
                        swaps.push(c0.clone());
                        swaps.push(real_c0.clone());
                        c0 = real_c0;
                        inst.out
                    }
                    None => match get_inst1(instructions, &c0, &Op::And) {
                        Some(inst) => {
                            let real_axorb = if inst.in1 == c0 { inst.in2 } else { inst.in1 };
                            swaps.push(axorb.clone());
                            swaps.push(real_axorb.clone());
                            axorb = real_axorb;
                            inst.out
                        }
                        None => panic!("shit"),
                    },
                },
            };
            if axorbandc.starts_with("z") && !s.starts_with("z") {
                let swap = axorbandc;
                axorbandc = s;
                s = swap;
                swaps.push(axorbandc.clone());
                swaps.push(s.clone());
            }
            let mut c1 = get_inst(instructions, &axorbandc, &aandb, &Op::Or)
                .unwrap()
                .out;
            if c1.starts_with("z") && !s.starts_with("z") {
                let swap = c1;
                c1 = s;
                s = swap;
                swaps.push(c1.clone());
                swaps.push(s.clone());
            }

            carry.insert(i, c1);
        }
    }

    swaps.sort();
    println!("part 2: {}", swaps.join(","));
}

fn get_inst(instructions: &Vec<Instruction>, x: &str, y: &str, op: &Op) -> Option<Instruction> {
    instructions
        .iter()
        .filter(|&inst| {
            &inst.op == op && ((inst.in1 == x && inst.in2 == y) || (inst.in2 == x || inst.in1 == y))
        })
        .map(|inst| inst)
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .cloned()
}
fn get_inst1(instructions: &Vec<Instruction>, x: &str, op: &Op) -> Option<Instruction> {
    instructions
        .iter()
        .filter(|&inst| &inst.op == op && ((inst.in1 == x) || (inst.in2 == x)))
        .map(|inst| inst)
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .cloned()
}
