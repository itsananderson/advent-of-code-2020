use std::env;
use std::fs;
use std::string::String;
use std::process::exit;
use std::collections::HashMap;

#[derive(Debug)]
struct Field {
    name: String,
    valid_ranges: [[u32; 2]; 2],
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines().collect::<Vec<&str>>();

    let mut rules: Vec<Field> = Vec::new();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();

    let mut i = 0;
    while split[i] != "" {
        let parts = split[i].split(": ").collect::<Vec<&str>>();
        let field_name = parts[0].to_string();
        let range_parts = parts[1].split(" or ").map(|part| part.split("-").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
        
        rules.push(Field {
            name: field_name,
            valid_ranges: [
                [range_parts[0][0], range_parts[0][1]],
                [range_parts[1][0], range_parts[1][1]]
            ],
        });

        i += 1;
    }

    println!("{:?}", rules);

    i += 1; // your ticket:
    i += 1; // values
    i += 1; //
    i += 1; // nearby tickets;
    i += 1; // nearby ticket values

    while i < split.len() {
        // println!("{}", split[i]);
        let values = split[i].split(",").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        // println!("{:?}", values);
        nearby_tickets.push(values);
        i += 1;
    }

    let mut sum = 0;

    for ticket in nearby_tickets {
        for value in ticket {
            if !is_valid_value(value, &rules) {
                println!("!{}", value);
                sum += value;
            }
        }
    }

    println!("TOTAL: {}", sum);
}

fn is_valid_value(value: u32, rules: &Vec<Field>) -> bool {
    for rule in rules {
        if value >= rule.valid_ranges[0][0] && value <= rule.valid_ranges[0][1] {
            return true;
        }

        if value >= rule.valid_ranges[1][0] && value <= rule.valid_ranges[1][1] {
            return true;
        }
    }

    return false;
}