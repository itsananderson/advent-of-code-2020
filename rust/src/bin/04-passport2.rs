
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
                "byr" => {
                    let year = value.parse::<i32>().unwrap();
                    if year >= 1920 && year <= 2002 { 1 } else { 0 }
                },
                "iyr" => {
                    let year = value.parse::<i32>().unwrap();
                    if year >= 2010 && year <= 2020 { 1 } else { 0 }
                },
                "eyr" => {
                    let year = value.parse::<i32>().unwrap();
                    if year >= 2020 && year <= 2030 { 1 } else { 0 }
                },
                "hgt" => {
                    let unit = &value[value.len()-2..value.len()];
                    if match unit {
                        "cm" => { 
                        let number = value[0..value.len()-2].parse::<i32>().unwrap();
                            number >= 150 && number <= 193 },
                        "in" => { 
                        let number = value[0..value.len()-2].parse::<i32>().unwrap();
                            number >= 59 && number <= 76 },
                        _ => false,
                    } { 1 } else { 0 }
                },
                "hcl" => {
                    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                    if re.is_match(value) { 1 } else { 0 }
                },
                "ecl" => {
                    match value {
                        "amb" => 1,
                        "blu" => 1,
                        "brn" => 1,
                        "gry" => 1,
                        "grn" => 1,
                        "hzl" => 1,
                        "oth" => 1,
                        _ => 0,
                    }
                },
                "pid" => {
                    let re = Regex::new(r"^[0-9]{9}$").unwrap();
                    if re.is_match(value) { 1 } else { 0 }
                },
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