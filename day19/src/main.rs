use clap::{crate_description, App, Arg};
use day19::BeamScanner;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());
    let program = read_input(args.value_of("INPUT").unwrap());
    let mut scanner = BeamScanner::new(&program);

    if let Some(result) = scanner.scan_and_count(50, 50) {
        println!("Part 1: {}", result);
    } else {
        println!("Failed failed to scan area");
        exit(4);
    }

    if let Some(result) = scanner.find_fit(100, 100) {
        println!("Part 2: {}", result);
    } else {
        println!("Failed failed to scan area");
        exit(4);
    }
}

fn read_input(filename: &str) -> Vec<i64> {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(2);
    });
    input
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|err: ParseIntError| {
            println!("Failed to parse input: {}", err.to_string());
            exit(3);
        })
}
