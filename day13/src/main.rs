use clap::{crate_description, value_t_or_exit, App, Arg};
use day13::{part1, part2};
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
        .arg(
            Arg::with_name("DISPLAY")
                .help("Enable display of gameplay")
                .short("d")
                .long("display"),
        )
        .arg(
            Arg::with_name("FRAME_TIME")
                .help("Frame time in milliseconds")
                .short("t")
                .long("frame-time")
                .takes_value(true)
                .default_value("15"),
        )
        .get_matches();

    println!(crate_description!());
    let program = read_input(args.value_of("INPUT").unwrap());
    let display = args.is_present("DISPLAY");
    let frametime = value_t_or_exit!(args.value_of("FRAME_TIME"), u64);
    if let Some(count) = part1(&program) {
        println!("Part 1: {}", count);
    } else {
        println!("Part 1: Program failed");
    }
    if let Some(score) = part2(&program, display, frametime) {
        println!("Part 2: {}", score);
    } else {
        println!("Part 2: Program failed");
    }
}

fn read_input(filename: &str) -> Vec<i64> {
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
