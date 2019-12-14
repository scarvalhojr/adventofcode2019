use num_integer::Integer;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Dimension {
    pos: i16,
    vel: i16,
}

impl Dimension {
    fn apply_gravity(&mut self, other: Dimension) {
        if self.pos < other.pos {
            self.vel += 1;
        } else if self.pos > other.pos {
            self.vel -= 1;
        }
    }

    fn update_position(&mut self) {
        self.pos += self.vel;
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Object {
    dim_x: Dimension,
    dim_y: Dimension,
    dim_z: Dimension,
}

impl Object {
    fn apply_gravity(&mut self, other: &Object) {
        self.dim_x.apply_gravity(other.dim_x);
        self.dim_y.apply_gravity(other.dim_y);
        self.dim_z.apply_gravity(other.dim_z);
    }

    fn update_position(&mut self) {
        self.dim_x.update_position();
        self.dim_y.update_position();
        self.dim_z.update_position();
    }

    fn total_energy(&self) -> u32 {
        (self.dim_x.pos.abs() as u32
            + self.dim_y.pos.abs() as u32
            + self.dim_z.pos.abs() as u32)
            * (self.dim_x.vel.abs() as u32
                + self.dim_y.vel.abs() as u32
                + self.dim_z.vel.abs() as u32)
    }
}

pub fn part1(moons: &[Object]) -> u32 {
    let mut moons = moons.iter().copied().collect::<VecDeque<Object>>();
    for _time in 1..=1_000 {
        for _ in 0..moons.len() {
            let mut moon = moons.pop_front().unwrap();
            for other in moons.iter() {
                moon.apply_gravity(other);
            }
            moons.push_back(moon);
        }
        for moon in moons.iter_mut() {
            moon.update_position();
        }
    }
    moons.iter().map(|moon| moon.total_energy()).sum()
}

fn find_repeat_cycle(dimensions: &[Dimension]) -> Option<usize> {
    let mut state = HashSet::new();
    let mut dimensions = dimensions.iter().copied().collect::<VecDeque<_>>();
    for time in 0.. {
        if !state.insert(dimensions.clone()) {
            return Some(time);
        }
        for _ in 0..dimensions.len() {
            let mut dim = dimensions.pop_front()?;
            for &other in dimensions.iter() {
                dim.apply_gravity(other);
            }
            dimensions.push_back(dim);
        }
        for dim in dimensions.iter_mut() {
            dim.update_position();
        }
    }
    None
}

pub fn part2(moons: &[Object]) -> Option<usize> {
    let x_dim = moons.iter().map(|moon| moon.dim_x).collect::<Vec<_>>();
    let y_dim = moons.iter().map(|moon| moon.dim_y).collect::<Vec<_>>();
    let z_dim = moons.iter().map(|moon| moon.dim_z).collect::<Vec<_>>();
    let repeat_x = find_repeat_cycle(&x_dim)?;
    let repeat_y = find_repeat_cycle(&y_dim)?;
    let repeat_z = find_repeat_cycle(&z_dim)?;
    Some(repeat_x.lcm(&repeat_y).lcm(&repeat_z))
}

impl FromStr for Object {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$")
            .unwrap()
            .captures(s)
            .ok_or_else(|| "Invalid input")?;
        let coord = captures
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "invalid coordinates".to_string())?;
        Ok(Self {
            dim_x: Dimension {
                pos: coord[0],
                vel: 0,
            },
            dim_y: Dimension {
                pos: coord[1],
                vel: 0,
            },
            dim_z: Dimension {
                pos: coord[2],
                vel: 0,
            },
        })
    }
}
