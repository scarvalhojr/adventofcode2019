use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use Move::*;

// Drop unnecesaary derives
#[derive(Clone, Debug)]
enum Move {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

// Drop unnecesaary derives
#[derive(Clone, Debug)]
pub struct Path {
    moves: Vec<Move>,
}

fn manhattan_dist(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

pub fn part1(path1: &Path, path2: &Path) -> i32 {
    let mut pos_x: i32  = 0;
    let mut pos_y: i32  = 0;
    let mut visited = HashSet::new();
    for m in path1.moves.iter() {
        match m {
            Left(dist) => {
                for x in (pos_x + 1)..=(pos_x + dist) {
                    visited.insert((x, pos_y));
                }
                pos_x += dist;
            },
            Right(dist) => {
                for x in (pos_x - dist)..pos_x {
                    visited.insert((x, pos_y));
                }
                pos_x -= dist;
            },
            Up(dist) => {
                for y in (pos_y + 1)..=(pos_y + dist) {
                    visited.insert((pos_x, y));
                }
                pos_y += dist;
            },
            Down(dist) => {
                for y in (pos_y - dist)..pos_y {
                    visited.insert((pos_x, y));
                }
                pos_y -= dist;
            },
        }
    }

    pos_x = 0;
    pos_y = 0;
    let mut closest = i32::max_value();
    for m in path2.moves.iter() {
        match m {
            Left(dist) => {
                for x in (pos_x + 1)..=(pos_x + dist) {
                    if visited.contains(&(x, pos_y)) {
                        let int_dist = manhattan_dist(x, pos_y);
                        if int_dist > 0 && int_dist < closest {
                            closest = int_dist;
                        }
                    }
                }
                pos_x += dist;
            },
            Right(dist) => {
                for x in (pos_x - dist)..pos_x {
                    if visited.contains(&(x, pos_y)) {
                        let int_dist = manhattan_dist(x, pos_y);
                        if int_dist > 0 && int_dist < closest {
                            closest = int_dist;
                        }
                    }
                }
                pos_x -= dist;
            },
            Up(dist) => {
                for y in (pos_y + 1)..=(pos_y + dist) {
                    if visited.contains(&(pos_x, y)) {
                        let int_dist = manhattan_dist(pos_x, y);
                        if int_dist > 0 && int_dist < closest {
                            closest = int_dist;
                        }
                    }
                }
                pos_y += dist;
            },
            Down(dist) => {
                for y in (pos_y - dist)..pos_y {
                    if visited.contains(&(pos_x, y)) {
                        let int_dist = manhattan_dist(pos_x, y);
                        if int_dist > 0 && int_dist < closest {
                            closest = int_dist;
                        }
                    }
                }
                pos_y -= dist;
            },
        }
    }

    closest
}

pub fn part2(path1: &Path, path2: &Path) -> i32 {
    let mut pos_x: i32  = 0;
    let mut pos_y: i32  = 0;
    let mut length = HashMap::new();
    let mut curr_len = 0;
    for m in path1.moves.iter() {
        match m {
            Left(dist) => {
                for x in (pos_x + 1)..=(pos_x + dist) {
                    curr_len += 1;
                    length.insert((x, pos_y), curr_len);
                }
                pos_x += dist;
            },
            Right(dist) => {
                for x in ((pos_x - dist)..pos_x).rev() {
                    curr_len += 1;
                    length.insert((x, pos_y), curr_len);
                }
                pos_x -= dist;
            },
            Up(dist) => {
                for y in (pos_y + 1)..=(pos_y + dist) {
                    curr_len += 1;
                    length.insert((pos_x, y), curr_len);
                }
                pos_y += dist;
            },
            Down(dist) => {
                for y in ((pos_y - dist)..pos_y).rev() {
                    curr_len += 1;
                    length.insert((pos_x, y), curr_len);
                }
                pos_y -= dist;
            },
        }
    }

    pos_x = 0;
    pos_y = 0;
    curr_len = 0;
    let mut closest = i32::max_value();
    for m in path2.moves.iter() {
        match m {
            Left(dist) => {
                for x in (pos_x + 1)..=(pos_x + dist) {
                    curr_len += 1;
                    if let Some(steps) = length.get(&(x, pos_y)) {
                        if steps + curr_len < closest {
                            closest = steps + curr_len;
                        }
                    }
                }
                pos_x += dist;
            },
            Right(dist) => {
                for x in (pos_x - dist)..pos_x {
                    curr_len += 1;
                    if let Some(steps) = length.get(&(x, pos_y)) {
                        if steps + curr_len < closest {
                            closest = steps + curr_len;
                        }
                    }
                }
                pos_x -= dist;
            },
            Up(dist) => {
                for y in (pos_y + 1)..=(pos_y + dist) {
                    curr_len += 1;
                    if let Some(steps) = length.get(&(pos_x, y)) {
                        if steps + curr_len < closest {
                            closest = steps + curr_len;
                        }
                    }
                }
                pos_y += dist;
            },
            Down(dist) => {
                for y in (pos_y - dist)..pos_y {
                    curr_len += 1;
                    if let Some(steps) = length.get(&(pos_x, y)) {
                        if steps + curr_len < closest {
                            closest = steps + curr_len;
                        }
                    }
                }
                pos_y -= dist;
            },
        }
    }

    closest
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
