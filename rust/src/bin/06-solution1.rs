use std::env;
use std::fs;
use std::string::String;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut totalGroupYes = 0;

    for group in split {
        let mut groupQuestions = HashSet::new();

        for person in group.lines().collect::<Vec<&str>>() {
            for answer in person.chars() {
                // print!("'{}'", answer);
                groupQuestions.insert(answer);
            }
        }

        // println!("{}\n-------", groupQuestions.len());
        totalGroupYes += groupQuestions.len();
    }

    println!("{}", totalGroupYes);
}