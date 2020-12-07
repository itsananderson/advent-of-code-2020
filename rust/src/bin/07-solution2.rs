

use std::env;
use std::fs;
use std::string::String;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct BagDescription {
    count: usize,
    description: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let charsToStrip = [
        ",", "\\."
    ];

    for ch in &charsToStrip {
        contents = Regex::new(ch).unwrap().replace_all(&contents, "").to_string();
    }

    contents = Regex::new("bag ").unwrap().replace_all(&contents, "bags ").to_string();

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut bagChildren: HashMap<String, Vec<BagDescription>> = HashMap::new();

    for line in lines {
        let parts = line.split(" contain ").collect::<Vec<&str>>();

        let outerPieces = parts[0].split(" ").collect::<Vec<&str>>();
        let outer = BagDescription {
            count: 1,
            description: format!("{} {}", outerPieces[0], outerPieces[1]),
        };

        println!("{:?}", outer);

        let iBags = match parts[1] {
            "no other bags" => Vec::new(),
            _ => {
                let innerPieces = parts[1].split(" ").collect::<Vec<&str>>();
                let mut innerBags: Vec<BagDescription> = Vec::new();

                let mut i = 0;
                while i < innerPieces.len() {
                    innerBags.push(BagDescription {
                        count: innerPieces[i].parse::<usize>().unwrap(),
                        description: format!("{} {}", innerPieces[i+1], innerPieces[i+2]),
                    });
                    i += 4;
                }
                println!("{:?}", innerBags);
                innerBags
            }
        };

        bagChildren.insert(outer.description, iBags);

    }

    println!("{:?}", bagChildren);

    // Subtract 1 for the "shiny yellow" entry
    println!("{}", bagCount(&bagChildren, "shiny gold".to_string())-1);
}

fn bagCount(bagChildren: &HashMap<String, Vec<BagDescription>>, bagDescription: String) -> usize {
    let mut runningCount: usize = 1;

    match bagChildren.get(&bagDescription) {
        Some(children) => {
            for child in children {
                runningCount += child.count * bagCount(&bagChildren, child.description.clone());
            }
        },
        None => {}
    }

    println!("{}", runningCount);
    runningCount
}