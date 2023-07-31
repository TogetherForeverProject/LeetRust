use std::env;
mod problems;
use crate::problems::problem120;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <problem_number>", args[0]);
        return;
    }

    // Get the problem number from the first argument
    let problem_number: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid problem number provided.");
            return;
        }
    };

    // Call the appropriate problem's solve() function based on the problem_number
    match problem_number {
        120 => problem120::solve(),
        _ => println!("Unknown problem number."),
    }
}
