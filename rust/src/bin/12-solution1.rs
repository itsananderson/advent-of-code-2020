use std::env;
use std::fs;
use std::string::String;
use std::process::exit;

#[derive(Debug)]
#[derive(Clone)]
enum Direction {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
    UNKNOWN,
}

#[derive(Debug)]
#[derive(Clone)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let instructions = split.map(|l| {
        let directionStr = &l[0..1];
        let amount = l[1..].parse::<i32>().unwrap();

        Instruction {
            direction: match directionStr {
                "N" => Direction::N,
                "S" => Direction::S,
                "E" => Direction::E,
                "W" => Direction::W,
                "L" => Direction::L,
                "R" => Direction::R,
                "F" => Direction::F,
                _ => Direction::UNKNOWN,
            },
            amount: amount,
        }
    });

    let mut currentDirection = Direction::E;
    let mut delta: [i32; 2] = [1, 0];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    
    for i in instructions {
        match i.direction {
            Direction::N => {
                y += i.amount;
            },
            Direction::W => {
                x -= i.amount;
            },
            Direction::E => {
                x += i.amount;
            },
            Direction::S => {
                y -= i.amount;
            },
            Direction::L => {
                currentDirection = nextDirection(&currentDirection, i.direction, i.amount);
                delta = directionDelta(&currentDirection);
            }
            Direction::R => {
                currentDirection = nextDirection(&currentDirection, i.direction, i.amount);
                delta = directionDelta(&currentDirection);
            },
            Direction::F => {
                x += delta[0] * i.amount;
                y += delta[1] * i.amount;
            },
            Direction::UNKNOWN => {
                println!("WTF UNKNOWN {:?}", i);
                exit(1);
            }
        }

        println!("({},{}) {:?}, {:?}", x, y, currentDirection, delta);
    }

    println!("{} + {} = {}", x.abs(), y.abs(), x.abs() + y.abs());
}

fn nextDirection(cur: &Direction, turnDirection: Direction, amount: i32) -> Direction {
    let mut nextDir = cur.clone();
    for i in 0..amount/90 {
        nextDir = nextDirectionHelper(&nextDir, &turnDirection);
    }
    return nextDir;
}

fn nextDirectionHelper(cur: &Direction, turnDirection: &Direction) -> Direction {
    match turnDirection {
        Direction::L => {
            return match cur {
                Direction::N => Direction::W,
                Direction::W => Direction::S,
                Direction::S => Direction::E,
                Direction::E => Direction::N,
                _ => {
                    println!("F Left {:?}", cur);
                    exit(1);
                }
            }
        },
        Direction::R => {
            return match cur {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
                _ => {
                    println!("F Right {:?}", cur);
                    exit(1);
                }
            }
        },
        _ => {
            println!("F {:?}", turnDirection);
            exit(1);
        }
    }
}

fn directionDelta(direction: &Direction) -> [i32; 2] {
    return match direction {
        Direction::W => [-1, 0],
        Direction::E => [1, 0],
        Direction::N => [0, 1],
        Direction::S => [0, -1],
        _ => {
            println!("WTF directionDelta {:?}", direction);
            exit(1);
        }
    };
}