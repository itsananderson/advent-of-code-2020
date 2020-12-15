
use std::env;
use std::fs;
use std::string::String;
use std::process::exit;
use std::panic;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.split(",").map(|s| { s.parse::<u64>().unwrap() }).collect::<Vec<u64>>();

    println!("{:?}", split);

    let mut last_seen: HashMap<u64, u64> = HashMap::new();
    let mut second_last_seen: HashMap<u64, u64> = HashMap::new();
    let mut last_num: u64 = 0;

    for (i, n) in split.iter().enumerate() {
        last_seen.insert(n.clone(), i as u64);
        last_num = *n;
        println!("{}", n);
    }

    for i in split.len()..30000000 {
        match last_seen.get_mut(&last_num) {
            Some(num) => {
                match second_last_seen.get_mut(&last_num) {
                    Some(num2) => {
                        // Seen the number before. Calculate delta
                        // println!("a {} {}", num, num2);
                        let next_last_num = num.clone() - num2.clone();

                        set_last_seen(&mut last_seen, &mut second_last_seen, next_last_num, i as u64);
                        last_num = next_last_num;
                    },
                    None => {
                        // Only seen the number once
                        set_last_seen(&mut last_seen, &mut second_last_seen, 0, i as u64);
                        last_num = 0;
                    }
                }
            },
            None => {
                panic!("WTF {} {} {:?} {:?}", i, last_num, last_seen, second_last_seen);
            }
        }
        
        if (i % 10000000 == 0) {
            println!("{}, {:?} {:?}", last_num, last_seen, second_last_seen);
        }
    }


    println!("{}", last_num);
}

fn set_last_seen(last_seen: &mut HashMap<u64, u64>, second_last_seen: &mut HashMap<u64, u64>, item: u64, i: u64) {
    match last_seen.get(&item) {
        Some(num) => {
            second_last_seen.insert(item, num.clone());
        },
        None => {}
    };
    last_seen.insert(item, i);
}