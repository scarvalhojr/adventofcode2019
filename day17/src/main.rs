use clap::{crate_description, App, Arg};
use day17::{ScaffoldScanner, ScaffoldWalker};
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

    let mut scanner = ScaffoldScanner::default();
    scanner.set_video_feed(video_feed);
    if let Some(alignment_params) = scanner.scan(&program) {
        println!("Part 1: {}", alignment_params);
    } else {
        println!("Failed failed to scan area");
        exit(4);
    }

    let path = scanner.build_scaffold_path();
    let mut walker = ScaffoldWalker::default();
    walker.set_video_feed(video_feed);
    if let Some(dust) = walker.walk(&program, &path) {
        println!("Part 2: {}", dust);
    } else {
        println!("Part 2: Failed to walk the scaffold");
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
