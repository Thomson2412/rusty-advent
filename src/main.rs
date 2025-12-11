use std::env;
use std::fs;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let day = if args.len() > 1 {
        args[1].parse::<u32>().expect("Please provide a valid day number")
    } else {
        println!("Usage: cargo run <day>");
        return;
    };

    let input = read_input(day);
    
    match day {
        1 => {
            println!("Day 1:");
            println!("Answer: {}", day01::execute(&input));
        }
        _ => println!("Day {} not implemented yet", day),
    }
}

fn read_input(day: u32) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Could not read file: {}", filename))
}