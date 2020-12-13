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

    let mut earliestRoute = std::i32::MAX;
    let mut earliestRouteIdx = -1;

    for route in routes {
        match route {
            "x" => {},
            _ => {
                let num = route.parse::<i32>().unwrap();

                println!("{} {}", num - (start % num), route);
            }
        }
    }
}