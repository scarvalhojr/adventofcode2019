use clap::{crate_description, App, Arg};
use day18::VaultMap;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT1")
                .help("Sets the input file to use for part 1")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT2")
                .help("Sets the input file to use for part 2")
                .required(true)
                .index(2),
        )
        .get_matches();

    println!(crate_description!());
    let vault_map1 = read_input(args.value_of("INPUT1").unwrap());
    if let Some(distance) = vault_map1.shortest_path_to_all_keys() {
        println!("Part 1: {}", distance);
    } else {
        println!("Part 1: a path to all keys was not found");
    }
    let vault_map2 = read_input(args.value_of("INPUT2").unwrap());
    if let Some(distance) = vault_map2.shortest_path_to_all_keys() {
        println!("Part 2: {}", distance);
    } else {
        println!("Part 2: a path to all keys was not found");
    }
}

fn read_input(filename: &str) -> VaultMap {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(2);
    });
    input.parse().unwrap_or_else(|err: String| {
        println!("Failed to parse input: {}", err.to_string());
        exit(3);
    })
}
