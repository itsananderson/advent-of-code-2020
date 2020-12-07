

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

    let mut bagParents: HashMap<String, Vec<String>> = HashMap::new();

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

        for bag in iBags {
            match bagParents.get_mut(&bag.description) {
                Some(bags) => {
                    bags.push(outer.description.clone());
                },
                None => {
                    let mut bags = Vec::new();
                    bags.push(outer.description.clone());
                    bagParents.insert(bag.description, bags);
                }
            }
        }
    }

    println!("{:?}", bagParents);

    let mut checked = HashSet::new();
    let mut toCheck = Vec::new();
    toCheck.push("shiny gold".to_string());

    while toCheck.len() > 0 {
        let item = toCheck.pop().unwrap();
        checked.insert(item.clone());

        match bagParents.get(&item) {
            Some(parents) => {
                for parent in parents {
                    if !checked.contains(parent) {
                        toCheck.push(parent.clone());
                        checked.insert(parent.to_string());
                    }
                }
            },
            None => {}
        }
    }

    // Subtract 1 for the "shiny gold" entry
    println!("{}", checked.len()-1);
}