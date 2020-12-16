use std::env;
use std::fs;
use std::string::String;
use std::process::exit;
use std::collections::HashMap;

#[derive(Clone)]
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
    i += 1; // <value>
    my_ticket = split[i].split(",").map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>();
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

    let valid_tickets = nearby_tickets.iter().filter(|t| {
        for value in t.iter() {
            if !is_valid_value(value.clone(), &rules) {
                return false;
            }
        }
        return true;
    }).collect::<Vec<&Vec<u32>>>();

    let mut matching_fields = (0..my_ticket.len()).map(|i| {
        return rules.clone();
    }).collect::<Vec<Vec<Field>>>();

    // println!("{:?}", matching_fields);

    for ticket in &valid_tickets {
        for (i, value) in ticket.iter().enumerate() {
            println!("{} {}", i, value);
            matching_fields[i] = matching_fields[i].iter().filter(|f| {
                return value_matches_rule(value.clone(), f);
            }).cloned().collect::<Vec<Field>>();
        }

        println!("{:?}", matching_fields);
        println!("{:?}", matching_fields.iter().map(|f| f.len()).collect::<Vec<usize>>());
    }

    let mut removing_field = "".to_string();

    let mut ticket_sum: i64 = 1;

    while matching_fields.iter().any(|f| f.len() > 1) {
        if !removing_field.is_empty() {
            for i in 0..matching_fields.len() {
                matching_fields[i] = matching_fields[i].iter().filter(|f| {
                    return f.name != removing_field;
                }).cloned().collect::<Vec<Field>>();
            }
        }

        for (i, fields) in matching_fields.iter().enumerate() {
            if fields.len() == 1 {
                removing_field = fields[0].name.clone();
                if removing_field.starts_with("departure") {
                    println!("{} {} {}", i, removing_field, my_ticket[i]);
                    ticket_sum *= my_ticket[i] as i64;
                }
            }
        }
    }

    println!("{}", ticket_sum);

    println!("{:?}", matching_fields.iter().map(|f| f.len()).collect::<Vec<usize>>());

    println!("TOTAL: {} {}", valid_tickets.len(), nearby_tickets.len());
}

fn is_valid_value(value: u32, rules: &Vec<Field>) -> bool {
    for rule in rules {
        if value_matches_rule(value, rule) {
            return true;
        }
    }

    return false;
}

fn value_matches_rule(value: u32, rule: &Field) -> bool {
    if value >= rule.valid_ranges[0][0] && value <= rule.valid_ranges[0][1] {
        return true;
    }

    if value >= rule.valid_ranges[1][0] && value <= rule.valid_ranges[1][1] {
        return true;
    }
    return false;
}