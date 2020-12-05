
use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let mut ids: Vec<i32> = split.map(|row| {
        let line = row.to_string();
        let mut row: i32 = 0;
        let mut col: i32 = 0;
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
        let id: i32 = row * 8 + col;
        return id;
    }).collect::<Vec<i32>>();

    ids.sort_by(|a,b| b.cmp(a));

    let mut last = 0;
    for id in ids {
        if id+1 != last {
            println!("{} {}", last, id);
        }

        last = id;
    }
}