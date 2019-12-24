use clap::{crate_description, App, Arg};
use day25::part1;
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
            Arg::with_name("VIDEO_FEED")
                .help("Enable the video feed")
                .short("v")
                .long("video-feed"),
        )
        .get_matches();

    println!(crate_description!());
    let program = read_input(args.value_of("INPUT").unwrap());
    let video_feed = args.is_present("VIDEO_FEED");

    if let Some(password) = part1(&program, video_feed) {
        println!("Part 1: {}", password);
    } else {
        println!("Program failed");
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
