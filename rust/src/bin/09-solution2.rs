use std::env;
use std::fs;
use std::string::String;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let target_sum = args[2].parse::<usize>().unwrap();

    println!("Looking for ranges In file {} with sum {}", filename, target_sum);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines();

    let nums: Vec<usize> = split.map(|line| line.parse::<usize>().unwrap()).collect();
    let len = nums.len() as i32;

    for i in 0..nums.len() {
        let mut sum = nums[i];

        for j in i+1..nums.len() {
            sum += nums[j];

            if sum > target_sum {
                break;
            } else if sum == target_sum {
                for n in i..=j {
                    println!("{}", nums[n]);
                }
                exit(0);
            }
        }
    }
}