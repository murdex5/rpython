use std::{env, process::exit};

fn main() {
    // Handles user input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./rpython <filename.rpy>");
        exit(0);
    }
    
    let last_four: String = args[1].chars().rev().take(4).collect::<String>().chars().rev().collect();
    if last_four != ".rpy" {
        println!("filetype should be .rpy");
        exit(0);
    }

}
