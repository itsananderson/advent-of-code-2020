
use std::env;
use std::fs;
use std::string::String;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    println!("{:?}", split.len());

    let mut validCount = 0;

    for passport in split {
        let parts = passport.split_whitespace().collect::<Vec<&str>>();

        println!("{:?}", parts);

        let mut validParts = 0;

        for part in parts {
            let key = part.split(":").collect::<Vec<&str>>()[0];
            let value = part.split(":").collect::<Vec<&str>>()[1];

            validParts += match key {
                "byr" => 1,
                "iyr" => 1,
                "eyr" => 1,
                "hgt" => 1,
                "hcl" => 1,
                "ecl" => 1,
                "pid" => 1,
                _ => 0,
            }
        }

        println!("{}", validParts);

        if validParts == 7 {
            validCount += 1;
        }
    }

    println!("Total valid: {}", validCount);
}