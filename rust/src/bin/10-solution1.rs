use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking at file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut split: Vec<i32> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    split.sort();

    println!("{:?}", split);

    let mut count1 = 1;
    let mut count2 = 0;
    let mut count3 = 1;

    let mut prv = split[0];

    for n in split {
        match n - prv {
            0 => {},
            1 => {
                count1 += 1;
            },
            2 => {
                count2 += 1;
            },
            3 => {
                count3 += 1;
            },
            _ => {
                println!("Unexpected Gap! {} {}", prv, n);
            }
        }
        prv = n;
    }

    println!("{} * {} = {}, ({})", count1, count3, count1 * count3, count2);
}