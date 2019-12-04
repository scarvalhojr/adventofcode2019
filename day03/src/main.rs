use clap::{crate_description, App, Arg};
use day03::{part1, part2, Path};
use std::fs::File;
use std::io::{BufRead, BufReader};
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
    let (path1, path2) = read_input(args.value_of("INPUT").unwrap());
    println!("Part 1: {}", part1(&path1, &path2));
    println!("Part 2: {}", part2(&path1, &path2));
}

fn read_input(filename: &str) -> (Path, Path) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file '{}': {}", filename, err.to_string());
            exit(2);
        }
    };

    match BufReader::new(file)
        .lines()
        .enumerate()
        .take(2)
        .map(|(num, line)| {
            line.map_err(|err| (num, err.to_string()))
                .and_then(|value| value.parse().map_err(|err| (num, err)))
        })
        .collect()
    {
        Ok(input) => input,
        Err((num, err)) => {
            println!("Failed to parse input file '{}'", filename);
            println!("Line {}: {}", num + 1, err);
            exit(3);
        }
    }
}
