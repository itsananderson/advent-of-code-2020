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

        printGrid(&cells);
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
            let adjacentSeats = fullLineOfSightSeats(grid, row, col);
            if adjacentSeats == 0 {
                return CellType::Full;
            } else {
                return CellType::Empty;
            }
        },
        CellType:: Full => {
            let adjacentSeats = fullLineOfSightSeats(grid, row, col);
            if adjacentSeats >= 5 {
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

fn fullLineOfSightSeats(grid: &Vec<Vec<CellType>>, row: usize, col: usize) -> usize {
    let mut fullSeats = 0;

    let gridWidth = grid.len() as i32;
    let gridHeight = grid[0].len() as i32;

    let deltas: [[i32; 2]; 8] = [
        [-1, -1], [-1,  0], [ 0, -1],
        [-1,  1], [ 1, -1], [ 1,  1],
        [ 0,  1], [ 1,  0]
    ];

    for d in deltas.iter() {
        let mut i: i32 = (row as i32) + d[0];
        let mut j: i32 = (col as i32) + d[1];

        // println!("{} {}", i, j);

        while i >= 0 && j >= 0 && i < gridWidth && j < gridHeight {
            if grid[i as usize][j as usize] == CellType::Full {
                fullSeats += 1;
                break;
            } else if grid[i as usize][j as usize] == CellType::Empty {
                break;
            }

            i += d[0];
            j += d[1];
        }
    }

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