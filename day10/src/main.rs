use clap::{crate_description, value_t_or_exit, App, Arg};
use day10::{part1, part2, AstroMap};
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
        .arg(
            Arg::with_name("TARGET")
                .help("Index of the nth asteroid to be vaporized")
                .required(true)
                .index(2),
        )
        .get_matches();

    println!(crate_description!());
    let astromap = read_input(args.value_of("INPUT").unwrap());
    let target = value_t_or_exit!(args.value_of("TARGET"), usize);
    if let Some((station, count)) = part1(&astromap) {
        println!("Part 1: {}", count);
        if let Some(result) = part2(&astromap, station, target) {
            println!("Part 2: {}", result);
        } else {
            println!("Part 2: target not found");
        }
    } else {
        println!("Part 1: location not found");
    }
}

fn read_input(filename: &str) -> AstroMap {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(3);
    });
    input.parse().unwrap_or_else(|err: String| {
        println!("Failed to parse input: {}", err.to_string());
        exit(3);
    })
}
