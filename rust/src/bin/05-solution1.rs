
use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let mut maxId = 0;

    for row in split {
        let line = row.to_string();
        let mut row = 0;
        let mut col = 0;
        for i in 0..7 {
            row += match &line[i..i+1] {
                "F" => 0,
                "B" => 1,
                _ => -1,
            };
            if i < 6 {
                row = row << 1;
            }
        }
        for j in 7..10 {
            col += match &line[j..j+1] {
                "R" => 1,
                "L" => 0,
                _ => -1,
            };
            if j != 9 {
                col = col << 1;
            }
        }
        let id = row * 8 + col;
        println!("{} {} {}, {}", line, row, col, id);
        
        if id > maxId {
            maxId = id;
        }
    }

    println!("{}", maxId)
}