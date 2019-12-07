use clap::{crate_description, value_t_or_exit, App, Arg};
use day04::{part1, part2};

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("RANGE_START")
                .help("Start of range")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("RANGE_END")
                .help("End of range")
                .required(true)
                .index(2),
        )
        .get_matches();

    println!(crate_description!());
    let range_start = value_t_or_exit!(args.value_of("RANGE_START"), u32);
    let range_end = value_t_or_exit!(args.value_of("RANGE_END"), u32);
    println!("Part 1: {}", part1(range_start, range_end));
    println!("Part 2: {}", part2(range_start, range_end));
}
