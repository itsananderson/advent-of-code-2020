use std::env;
use std::fs;
use std::string::String;
use std::process::exit;

#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[derive(Clone)]
enum CellType {
    Floor,
    Empty,
    Full,
    UNKNOWN
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let mut cells = Vec::new();

    for line in split {
        let row = line.chars().map(|c| {
            match c {
                'L' => CellType::Empty,
                '.' => CellType::Floor,
                '#' => CellType::Full,
                _   => {
                    println!("Unexpected char {}", c);
                    CellType::UNKNOWN
                }
            }
        }).collect::<Vec<CellType>>();
        cells.push(row);
    }

    // println!("{:?}", &cells);

    let mut changed = true;

    while changed {
        changed = false;
        let mut newGrid = Vec::new();
        for row in 0..((&cells).len()) {
            let mut newRow = Vec::new();
            for col in 0..((&cells[row]).len()) {
                let mutation = nextState(&cells, row, col);
                if mutation != cells[row][col] {
                    changed = true;
                    newRow.push(mutation);
                } else {
                    newRow.push(cells[row][col].clone());
                }
            }
            newGrid.push(newRow);
        }
        cells = newGrid;

        // printGrid(&cells);
    }

    let mut fullCount = 0;
    for row in 0..((&cells).len()) {
        for col in 0..((&cells[row]).len()) {
            if cells[row][col] == CellType::Full {
                fullCount += 1;
            }
        }
    }

    println!("fullCount = {}", fullCount);
}

fn nextState(grid: &Vec<Vec<CellType>>, row: usize, col: usize) -> CellType {
    match grid[row][col] {
        CellType::Floor => CellType::Floor,
        CellType::Empty => {
            let adjacentSeats = fullAdjacentSeats(grid, row, col);
            if adjacentSeats == 0 {
                return CellType::Full;
            } else {
                return CellType::Empty;
            }
        },
        CellType:: Full => {
            let adjacentSeats = fullAdjacentSeats(grid, row, col);
            if adjacentSeats >= 4 {
                return CellType::Empty;
            } else {
                return CellType::Full;
            }
        },
        CellType::UNKNOWN => {
            CellType::UNKNOWN
        }
    }
}

fn fullAdjacentSeats(grid: &Vec<Vec<CellType>>, row: usize, col: usize) -> usize {
    let mut fullSeats = 0;

    let rowStart = match row { 0 => 0, _ => row-1 };
    let colStart = match col { 0 => 0, _ => col-1 };

    for i in rowStart..=(row+1) {
        if i < 0 || i >= grid.len() {
            continue;
        }
        for j in colStart..=(col+1) {
            // println!("{},{}", i, j);
            if j < 0 || j >= grid[i].len() {
                continue;
            }

            if i == row && j == col {
                continue;
            }

            if grid[i][j] == CellType::Full {
                fullSeats += 1;
            }
        }
    }

    // println!("{},{} => {}", row, col, fullSeats);
    return fullSeats;
}

fn printGrid(grid: &Vec<Vec<CellType>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", match grid[i][j] {
                CellType::Full => '#',
                CellType::Empty => 'L',
                CellType::Floor => '.',
                _ => '?'
            })
        }
        println!("");
    }

    println!("-----------------------------------");
}