
use std::env;
use std::fs;
use std::string::String;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
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
        let low: usize = row.split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        let high: usize = row.split(" ").collect::<Vec<&str>>()[0].split('-').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
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

        for (i, c) in p.password.chars().enumerate() {
            if i + 1 == p.min || i + 1 == p.max {
                if c == p.letter {
                    char_count += 1;
                }
            }
        } 

        let good = char_count == 1;

        println!("count {}: {}", char_count, if good { "Good" } else { "Bad" });

        if good {
            good_passwords += 1;
        }
    }

    println!("Found {} good passwords", good_passwords)
}