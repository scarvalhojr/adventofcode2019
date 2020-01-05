use clap::{crate_description, App, Arg};
use day16::{part1, part2, FFT};
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
    let fft = read_input(args.value_of("INPUT").unwrap());
    println!("Part 1: {}", part1(&fft));
    println!("Part 2: {}", part2(&fft));
}

fn read_input(filename: &str) -> FFT {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(2);
    });
    input.parse().unwrap_or_else(|err| {
        println!("Failed to parse input: {}", err);
        exit(3);
    })
}
