
use std::env;
use std::fs;
use std::string::String;

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let passwords: Vec<Password> = split.map(|row| {
        let low: i32 = row.split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        let high: i32 = row.split(" ").collect::<Vec<&str>>()[0].split('-').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let letter = row.split(" ").collect::<Vec<&str>>()[1].chars().collect::<Vec<char>>()[0];
        let password = row.split(": ").collect::<Vec<&str>>()[1];
        Password {
            min: low,
            max: high,
            letter: letter,
            password: String::from(password),
        }
    }).collect();

    let mut good_passwords = 0;

    for p in passwords {
        println!("Password {:?}", p);

        let mut char_count = 0;

        for c in p.password.chars() {
            if c == p.letter {
                char_count += 1;
            }
        } 

        let good = char_count >= p.min && char_count <= p.max;

        println!("count {}: {}", char_count, if good { "Good" } else { "Bad" });

        if good {
            good_passwords += 1;
        }
    }

    println!("Found {} good passwords", good_passwords)
}