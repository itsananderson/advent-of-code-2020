use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let grid = split.map(|line| line.chars().map(|c| c != '.').collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    let mut x = 0;
    let mut y = 0;

    let mut accum: i64 = 1;

    let deltas = [[1,1], [3,1], [5,1], [7,1], [1,2]];

    for delta in &deltas {
        x = 0;
        y = 0;
        let dX = delta[0];
        let dY = delta[1];

        let modulo = grid[0].len();

        let mut trees = 0;

        while y < grid.len() {
            if grid[y][x] {
                // println!("{} {}", x, y);
                trees += 1;
            }

            x += dX;
            y += dY;

            if x >= modulo {
                x = x % modulo;
            }
        }

        println!("{}", trees);
        accum *= trees;
        println!("{}", accum);
    }

    println!("{}", accum);
}