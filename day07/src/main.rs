use clap::{crate_description, App, Arg};
use day07::{part1, part2};
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
    if let Some(result1) = part1(&program) {
        println!("Part 1: {}", result1);
    } else {
        println!("Part 1: Program failed");
    }
    if let Some(result1) = part2(&program) {
        println!("Part 2: {}", result1);
    } else {
        println!("Part 2: Program failed");
    }
}

fn read_input(filename: &str) -> Vec<i32> {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(3);
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
