use clap::{crate_description, App, Arg};
use day20::Maze;
use std::fs::read_to_string;
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
    let maze = read_input(args.value_of("INPUT").unwrap());
    if let Some(distance) = maze.shortest_path(false) {
        println!("Part 1: {}", distance);
    } else {
        println!("Part 1: path not found");
    }
    if let Some(distance) = maze.shortest_path(true) {
        println!("Part 2: {}", distance);
    } else {
        println!("Part 2: path not found");
    }
}

fn read_input(filename: &str) -> Maze {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(2);
    });
    input.parse().unwrap_or_else(|err: String| {
        println!("Failed to parse input: {}", err.to_string());
        exit(3);
    })
}
