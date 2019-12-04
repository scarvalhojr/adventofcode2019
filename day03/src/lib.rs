use std::str::FromStr;
use Move::*;

#[derive(Debug)]
enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug)]
pub struct Path {
    moves: Vec<Move>,
}

pub fn part1(path1: &Path, path2: &Path) -> usize {
    0
}

pub fn part2(path1: &Path, path2: &Path) -> usize {
    0
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.get(0..1).ok_or("Invalid move")?;
        let distance = s
            .get(1..)
            .unwrap()
            .parse()
            .map_err(|_| "Invalid distance")?;
        match direction {
            "L" => Ok(Left(distance)),
            "R" => Ok(Right(distance)),
            "U" => Ok(Up(distance)),
            "D" => Ok(Down(distance)),
            _ => Err("Invalid direction".to_string()),
        }
    }
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|move_str| move_str.parse())
            .collect::<Result<Vec<_>, _>>()
            .map(|moves| Path { moves })
    }
}
