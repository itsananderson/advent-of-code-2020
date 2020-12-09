use std::env;
use std::fs;
use std::string::String;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lookback = args[2].parse::<i32>().unwrap();

    println!("In file {} with lookback {}", filename, lookback);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let nums: Vec<i64> = split.map(|line| line.parse::<i64>().unwrap()).collect();
    let len = nums.len() as i32;

    for i in lookback..(nums.len() as i32) {
        let mut foundSum = false;
        for j in (i-lookback)..(i) {
            for k in (j+1)..(i) {
                if nums[i as usize] == nums[j as usize] + nums[k as usize] {
                    foundSum = true;
                    println!("{} {} {}", i, j, k);
                    println!("{} + {} = {}", nums[j as usize], nums[k as usize], nums[i as usize]);
                }
            }
        }

        if !foundSum {
            println!("{}", nums[i as usize]);
            exit(0);
        }
        println!("------------");
    }
}