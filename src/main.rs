use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use colored::*;

mod nixos;
mod nonnixos;

fn is_nixos() -> bool {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("ID=nixos") {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let argument = &args[1].to_string();
        let installion_argument = args.get(2).map(String::clone).unwrap_or_default();

        if is_nixos() {
            nixos::vix_nixos(argument, installion_argument);
        } else {
            nonnixos::non_nixos(argument, installion_argument);
        }
    } else {
        println!("{}: No argument provided", "Error".red());
    }
}
