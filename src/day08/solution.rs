use std::{collections::HashSet, fs::read_to_string, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Instruction {
    operation: Operation,
    argument: i32,
    line: usize,
}

pub fn find_boot(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let mut hash: HashSet<usize> = HashSet::new();

    let instructions: Vec<Instruction> = file
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, x)| {
            let operation = match &x[..=2] {
                "jmp" => Operation::Jmp,
                "acc" => Operation::Acc,
                "nop" => Operation::Nop,
                _ => unreachable!(),
            };

            let argument = match &x[4..5] {
                "+" => x[5..].parse::<i32>().unwrap(),
                "-" => -1 * x[5..].parse::<i32>().unwrap(),
                _ => unreachable!(),
            };

            Instruction {
                operation,
                argument,
                line: index,
            }
        })
        .collect();

    let mut accumulator = 0;
    let mut pointer = 0;

    loop {
        if hash.contains(&instructions[pointer].line) {
            println!("{}", accumulator);
            break;
        }

        let increment = match instructions[pointer].operation {
            Operation::Acc => {
                accumulator += instructions[pointer].argument;
                1
            }
            Operation::Jmp => instructions[pointer].argument,
            Operation::Nop => 1,
        };

        hash.insert(instructions[pointer].line);
        match increment {
            1..=std::i32::MAX => pointer += increment as usize,
            std::i32::MIN..=0 => pointer = (pointer as i32 + increment) as usize,
        }
    }
}

pub fn fix_boot(input: PathBuf) {
    let file = read_to_string(input).unwrap();

    let mut hash: HashSet<usize> = HashSet::new();

    let instructions: Vec<Instruction> = file
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, x)| {
            let operation = match &x[..=2] {
                "jmp" => Operation::Jmp,
                "acc" => Operation::Acc,
                "nop" => Operation::Nop,
                _ => unreachable!(),
            };

            let argument = match &x[4..5] {
                "+" => x[5..].parse::<i32>().unwrap(),
                "-" => -1 * x[5..].parse::<i32>().unwrap(),
                _ => unreachable!(),
            };

            Instruction {
                operation,
                argument,
                line: index,
            }
        })
        .collect();

    let mut accumulator = 0;
    let mut pointer = 0;
    let mut highest_line = 0;
    let mut jmps_and_nops = HashSet::new();

    while pointer <= instructions.len() - 1 {
        let increment = match instructions[pointer].operation {
            Operation::Acc => {
                accumulator += instructions[pointer].argument;
                1
            }
            Operation::Jmp => {
                if !jmps_and_nops.contains(&instructions[pointer]) {
                    jmps_and_nops.insert(instructions[pointer]);
                } else {
                    let mut fix = instructions[pointer].clone();
                    fix.operation = match fix.operation {
                        Operation::Jmp => Operation::Nop,
                        Operation::Nop => Operation::Jmp,
                        _ => break,
                    }
                }
                instructions[pointer].argument
            }
            Operation::Nop => {
                if !jmps_and_nops.contains(&instructions[pointer]) {
                    jmps_and_nops.insert(instructions[pointer]);
                } else {
                    let mut fix = instructions[pointer].clone();
                    fix.operation = match fix.operation {
                        Operation::Jmp => Operation::Nop,
                        Operation::Nop => Operation::Jmp,
                        _ => break,
                    }
                }
                1
            }
        };

        if instructions[pointer].line > highest_line {
            highest_line = instructions[pointer].line;
        }

        hash.insert(instructions[pointer].line);
        match increment {
            1..=std::i32::MAX => {
                pointer += increment as usize;
            }
            std::i32::MIN..=0 => pointer = (pointer as i32 + increment) as usize,
        }
    }
    println!("{}", accumulator);
}
