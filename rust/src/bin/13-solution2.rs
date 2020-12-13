use std::env;
use std::fs;
use std::string::String;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Looking for ranges In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split = contents.lines().collect::<Vec<&str>>();

    let start = split[0].parse::<i32>().unwrap();
    let routes = split[1].split(",").collect::<Vec<&str>>();

    println!("{:?}", routes);

    let mut earliestRoute = std::i64::MAX;
    let mut earliestRouteIdx = -1;

    let mut accum = 0;
    let mut inc = 1;

    for (i, route) in routes.iter().enumerate() {
        match route {
            &"x" => {},
            _ => {
                let num = route.parse::<i64>().unwrap();
                
                while (accum + i as i64) % num != 0{
                    accum += inc;

                    if false {
                        println!("i={}, num={}, accum={}, inc={}", i, num, accum, inc);
                    }
                }

                println!("i={}, num={}, accum={}, inc={}", i, num, accum, inc);

                // I think this only works because all the inputs are primes
                inc *= num;
            }
        }
    }
}