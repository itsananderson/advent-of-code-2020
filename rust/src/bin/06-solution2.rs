use std::env;
use std::fs;
use std::string::String;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut totalGroupYes = 0;

    for group in split {
        let mut groupQuestions: HashMap<char, usize> = HashMap::new();

        let people = group.lines().collect::<Vec<&str>>();

        let peopleCount = people.len();

        for person in people {
            for answer in person.chars() {
                groupQuestions.insert(answer, match groupQuestions.get(&answer) {
                    Some(value) => value + 1,
                    None => 1
                });
            }
        }

        println!("{:?}", groupQuestions);

        // println!("{}\n-------", groupQuestions.len());
        totalGroupYes += groupQuestions.values().filter(|v| v == &&peopleCount).collect::<Vec<&usize>>().len();
    }

    println!("{}", totalGroupYes);
}