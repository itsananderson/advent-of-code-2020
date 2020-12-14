
use std::env;
use std::fs;
use std::string::String;
use std::process::exit;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum MaskOperation {
    AND,
    OR,
}

// #[derive(Debug)]
#[derive(Clone)]
struct MaskValue {
    operation: MaskOperation,
    value: u64, 
}

impl std::fmt::Debug for MaskValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(op: {:?}, value: {:#064b})", self.operation, self.value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines().collect::<Vec<&str>>();

    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut masks: Vec<MaskValue> = Vec::new();

    for instruction in split {
        if &instruction[0..4] == "mask" {
            let mask = &instruction[7..];
            println!("{}, {}", mask, mask.len());
            masks = get_mask_vec(mask);
        } else {
            let parts = instruction.split("] = ").collect::<Vec<&str>>();
            let index = parts[0].split("[").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            let value = parts[1].parse::<u64>().unwrap();
            // println!("mem[{}] = {}", index, value);
            let masked_value = apply_masks(&masks, value);
            mem.insert(index, masked_value);
            println!("{}", masked_value);
        }
    }

    let mut sum: u64 = 0;

    for (key, value) in mem.iter() {
        sum += value;
    }

    println!("sum: {}", sum);
}

fn get_mask_vec(mask: &str) -> Vec<MaskValue> {
    let mut values: Vec<MaskValue> = Vec::new();
    
    for (i, c) in mask.chars().enumerate() {
        match c {
            '1' => {
                // println!("{}", 35-i);
                let mut value: u64 = 1 << (35-i);
                values.push(MaskValue {
                    operation: MaskOperation::OR,
                    value: value
                })
            },
            '0' => {
                // println!("{}", 35-i);
                let mut value = std::u64::MAX;
                let xor = 1 << (35-i);
                value = value ^ xor;
                values.push(MaskValue {
                    operation: MaskOperation::AND,
                    value: value
                })
            },
            'X' => {},
            _ => {
                println!("Unexpected mask value {}", c);
                exit(1);
            }
        }
    }

    values.reverse();
    return values;
}

fn apply_masks(masks: &Vec<MaskValue>, value: u64) -> u64 {
    let mut new_value = value;

    for mask in masks {
        // println!("new_value {}", new_value);
        match mask.operation {
            MaskOperation::AND => {
                // println!("AND {:#064b}", mask.value);
                new_value = new_value & mask.value;
            },
            MaskOperation::OR => {
                // println!("OR {:#064b}", mask.value);
                new_value = new_value | mask.value;
            }
        }
    }

    return new_value;
}