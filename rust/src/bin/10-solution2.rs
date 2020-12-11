use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking at file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut split: Vec<usize> = contents.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    split.sort();

    split.insert(0, 0);
    split.push(split[split.len()-1] + 3);

    println!("{:?}", split);

    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;

    let mut prv = &split[0];

    for (i, n) in split.iter().enumerate() {
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

    let mut gaps: Vec<usize> = Vec::new();

    for i in 1..(split.len()-1) {
        let n = split[i];
        let prv = split[i-1];
        let nxt = split[i+1];

        println!("{}({}) = {}", i, n, nxt-prv);
        gaps.push(nxt-prv);
    }

    let mut combos: i64 = 1;

    let mut inChunk = false;
    let mut chunkSize = 0;

    for g in &gaps {
        if g == &(2 as usize) {
            if inChunk {
                chunkSize += 1;
            } else {
                inChunk = true;
                chunkSize = 1;
            }
        } else {
            if inChunk {
                println!("{}", chunkSize);
                inChunk = false;

                match chunkSize {
                    1 => {
                        combos *= 2;
                    },
                    2 => {
                        combos *= 4;
                    },
                    3 => {
                        combos *= 7;
                    },
                    4 => {
                        combos *= 12;
                    },
                    _ => {
                        println!("Unexpected chunk size {}", chunkSize);
                    }
                }
            }
        }
    }

    println!("{:?}", gaps);
    println!("{}", combos);
}