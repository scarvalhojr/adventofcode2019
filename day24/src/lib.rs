use regex::Regex;
use std::str::FromStr;

pub struct Input {
    data: u32,
}

pub fn part1(input: &[Input]) -> u32 {
    input.get(0).map(|input| input.data).unwrap_or(0)
}

pub fn part2(_input: &[Input]) -> u32 {
    0
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"^(?P<input>\d+).*$")
            .unwrap()
            .captures(s)
            .ok_or_else(|| "Invalid input")?;

        captures
            .name("input")
            .unwrap()
            .as_str()
            .parse()
            .map(|data| Input { data })
            .map_err(|e| e.to_string())
    }
}
