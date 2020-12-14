
use std::env;
use std::fs;
use std::string::String;
use std::process::exit;
use std::collections::HashMap;

#[derive(Clone)]
struct Mask {
    or_mask: u64, 
    x_masks: Vec<[u64;2]>
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(or_mask: {:#064b}, ", self.or_mask);
        write!(f, "x_masks: [");
        for mask in &self.x_masks {
            write!(f, "\n[{:#064b}, {:#064b}], ", mask[0], mask[1]);
        }
        write!(f, "])")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines().collect::<Vec<&str>>();

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Mask = Mask {
        or_mask: 0,
        x_masks: Vec::new(),
    };

    for instruction in split {
        if &instruction[0..4] == "mask" {
            let mask_str = &instruction[7..];
            println!("{}, {}", mask_str, mask_str.len());
            let mutations = get_mask_mutations(mask_str);
            let or_mask = get_or_mask(mask_str);
            mask = Mask {
                or_mask: or_mask,
                x_masks: mutations
            };
            println!("{:?}", mask);
        } else {
            let parts = instruction.split("] = ").collect::<Vec<&str>>();
            let index = parts[0].split("[").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
            let value = parts[1].parse::<u64>().unwrap();
            for masked_index in apply_mask(&mask, index) {
                println!("{}", masked_index);
                mem.insert(masked_index, value);
            }
        }
    }

    let mut sum: u64 = 0;

    for (key, value) in mem.iter() {
        sum += value;
    }

    println!("sum: {}", sum);
}

fn get_or_mask(mask: &str) -> u64 {
    let mut mask_value: u64 = 0;

    for c in mask.chars() {
        match c {
            '1' => {
                mask_value = (mask_value << 1) + 1;
            },
            _ => {
                mask_value = mask_value << 1;
            }
        }
    }

    return mask_value;
}

fn get_mask_mutations(mask: &str) -> Vec<[u64; 2]> {
    if mask.len() == 0 {
        println!("end");
        let mut empty = Vec::new();
        empty.push([0, std::u64::MAX]);
        return empty;
    }
    let next_mask = &mask[0..mask.len()-1];
    // println!("{}", next_mask);

    let next_mutations = get_mask_mutations(next_mask);
    match &mask[(mask.len()-1)..mask.len()] {
        "X" => {
            let mut zero_mutations = next_mutations.iter().map(|m| [m[0] << 1, m[1] << 1]).collect::<Vec<[u64; 2]>>();
            let mut one_mutations = next_mutations.iter().map(|m| [(m[0] << 1) + 1, (m[1] << 1) + 1]).collect::<Vec<[u64; 2]>>();
            zero_mutations.append(&mut one_mutations);
            return zero_mutations;
        },
        _ => {
            return next_mutations.iter().map(|m| [m[0] << 1, (m[1] << 1) + 1]).collect::<Vec<[u64; 2]>>();
        }
    }
}

fn apply_mask(mask: &Mask, value: u64) -> Vec<u64> {
    let mut new_value = value | mask.or_mask;

    return mask.x_masks.iter().map(|x_mask| {
        (new_value | x_mask[0]) & x_mask[1]
    }).collect::<Vec<u64>>();
}