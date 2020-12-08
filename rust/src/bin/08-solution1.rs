

use std::env;
use std::fs;
use std::string::String;
use std::collections::HashSet;

#[derive(Debug)]
enum Operation {
    acc,
    jmp,
    nop,
    invalid,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    value: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let instructions: Vec<Instruction> = split.map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        return Instruction {
            operation: match parts[0] {
                "acc" => Operation::acc,
                "jmp" => Operation::jmp,
                "nop" => Operation::nop,
                _ => Operation::invalid,
            },
            value: parts[1].parse::<i32>().unwrap(),
        }
    }).collect();

    let mut visited = HashSet::new();
    let mut accum: i32 = 0;
    let mut inst: i32 = 0;

    while !visited.contains(&inst) {
        visited.insert(inst);

        let instruction = &instructions[inst as usize];

        println!("{:?} {}", instruction.operation, instruction.value);

        match instruction.operation {
            Operation::acc => {
                accum += instruction.value;
                inst += 1;
            },
            Operation::jmp => {
                inst += instruction.value;
            },
            Operation::nop => {
                inst += 1;
            },
            Operation::invalid => {
                println!("INVALID OPERATION");
            }
        }
    }

    println!("{}", accum);
}