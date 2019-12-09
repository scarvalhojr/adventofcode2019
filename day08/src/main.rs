use clap::{crate_description, value_t_or_exit, App, Arg};
use day08::{part1, Image};
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
            Arg::with_name("WIDTH")
                .help("Image width in pixels")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("HEIGHT")
                .help("Image height in pixels")
                .required(true)
                .index(3),
        )
        .get_matches();

    println!(crate_description!());
    let pixels = read_input(args.value_of("INPUT").unwrap());
    let height = value_t_or_exit!(args.value_of("HEIGHT"), usize);
    let width = value_t_or_exit!(args.value_of("WIDTH"), usize);
    let image = Image::new(height, width, &pixels);
    println!("Part 1: {}", part1(&image));
    println!("Part 2:\n{}", image.to_string());
}

fn read_input(filename: &str) -> Vec<u8> {
    let input = read_to_string(filename).unwrap_or_else(|err| {
        println!("Failed to read file '{}': {}", filename, err.to_string());
        exit(3);
    });
    input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_digit(10).map(|digit| digit as u8))
        .collect::<Option<Vec<_>>>()
        .unwrap_or_else(|| {
            println!("Invalid input");
            exit(3);
        })
}
