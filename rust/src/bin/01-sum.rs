use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.split_whitespace();

    let nums: Vec<i32> = split.map(|s| s.parse::<i32>().unwrap()).collect();

    for i in &nums {
        for j in &nums {
            if i + j == 2020 {
                println!("Found {} and {} => {}", i, j, i * j);
            }
        }
    }
}
