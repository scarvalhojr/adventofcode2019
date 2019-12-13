use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Asteroid {
    pos_x: i32,
    pos_y: i32,
}

impl Asteroid {
    fn new(pos_x: i32, pos_y: i32) -> Self {
        Self { pos_x, pos_y }
    }

    fn direction_to(self, other: Asteroid) -> Option<Direction> {
        Direction::new(other.pos_x - self.pos_x, other.pos_y - self.pos_y)
    }

    fn manhattan_distance_to(self, other: Asteroid) -> i32 {
        (other.pos_x - self.pos_x).abs() + (other.pos_y - self.pos_y).abs()
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Direction {
    delta_x: i32,
    delta_y: i32,
}

impl Direction {
    fn new(delta_x: i32, delta_y: i32) -> Option<Self> {
        match (delta_x.signum(), delta_y.signum()) {
            (0, 0) => None,
            (0, _) => Some(Self {
                delta_x,
                delta_y: delta_y.signum(),
            }),
            (_, 0) => Some(Self {
                delta_x: delta_x.signum(),
                delta_y,
            }),
            (_, _) => {
                let divisor = gcd(delta_x, delta_y);
                let dx = delta_x / divisor;
                let dy = delta_y / divisor;
                Some(Self {
                    delta_x: dx,
                    delta_y: dy,
                })
            }
        }
    }

    fn angle(&self) -> f32 {
        let a = (self.delta_y as f32).atan2(self.delta_x as f32)
            + std::f32::consts::FRAC_PI_2;
        if a < 0.0 {
            a + 2.0 * std::f32::consts::PI
        } else {
            a
        }
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle().partial_cmp(&other.angle()).unwrap()
    }
}

pub struct AstroMap {
    asteroids: HashSet<Asteroid>,
}

impl AstroMap {
    fn count_reachable_from(&self, asteroid: Asteroid) -> usize {
        self.asteroids
            .iter()
            .filter_map(|&other| asteroid.direction_to(other))
            .collect::<HashSet<_>>()
            .len()
    }

    fn find_best_station(&self) -> Option<(Asteroid, usize)> {
        self.asteroids
            .iter()
            .map(|asteroid| (asteroid, self.count_reachable_from(*asteroid)))
            .max_by_key(|(_, count)| *count)
            .map(|(asteroid, count)| (*asteroid, count))
    }

    fn find_nth_target(
        &self,
        station: Asteroid,
        index: usize,
    ) -> Option<Asteroid> {
        let mut targets: BTreeMap<Direction, BTreeMap<i32, Asteroid>> =
            BTreeMap::new();
        for asteroid in self.asteroids.iter() {
            if let Some(dir) = station.direction_to(*asteroid) {
                let dist = station.manhattan_distance_to(*asteroid);
                targets
                    .entry(dir)
                    .and_modify(|entry| {
                        entry.insert(dist, *asteroid);
                    })
                    .or_insert_with(|| {
                        [(dist, *asteroid)].iter().cloned().collect()
                    });
            }
        }
        let mut count = 1;
        loop {
            let mut all_vaporized = true;
            for (_, target) in targets.iter_mut() {
                let distance;
                if let Some((dist, _)) = target.iter_mut().next() {
                    distance = *dist;
                } else {
                    continue;
                }
                all_vaporized = false;
                let t = target.remove(&distance);
                if count == index {
                    return t;
                }
                count += 1;
            }
            if all_vaporized {
                break;
            }
        }
        None
    }
}

pub fn part1(astromap: &AstroMap) -> Option<(Asteroid, usize)> {
    astromap.find_best_station()
}

pub fn part2(
    astromap: &AstroMap,
    station: Asteroid,
    index: usize,
) -> Option<i32> {
    astromap
        .find_nth_target(station, index)
        .map(|asteroid| 100 * asteroid.pos_x + asteroid.pos_y)
}

impl FromStr for AstroMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asteroids = s
            .lines()
            .enumerate()
            .flat_map(|(pos_y, line)| {
                line.chars().enumerate().filter(|&(_, ch)| ch == '#').map(
                    move |(pos_x, _)| Asteroid::new(pos_x as i32, pos_y as i32),
                )
            })
            .collect();
        Ok(Self { asteroids })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let tests = [
            (
                ".#..#\n\
                 .....\n\
                 #####\n\
                 ....#\n\
                 ...##",
                Some((Asteroid::new(3, 4), 8)),
            ),
            (
                "......#.#.\n\
                 #..#.#....\n\
                 ..#######.\n\
                 .#.#.###..\n\
                 .#..#.....\n\
                 ..#....#.#\n\
                 #..#....#.\n\
                 .##.#..###\n\
                 ##...#..#.\n\
                 .#....####",
                Some((Asteroid::new(5, 8), 33)),
            ),
            (
                "#.#...#.#.\n\
                 .###....#.\n\
                 .#....#...\n\
                 ##.#.#.#.#\n\
                 ....#.#.#.\n\
                 .##..###.#\n\
                 ..#...##..\n\
                 ..##....##\n\
                 ......#...\n\
                 .####.###.",
                Some((Asteroid::new(1, 2), 35)),
            ),
            (
                ".#..#..###\n\
                 ####.###.#\n\
                 ....###.#.\n\
                 ..###.##.#\n\
                 ##.##.#.#.\n\
                 ....###..#\n\
                 ..#.#..#.#\n\
                 #..#.#.###\n\
                 .##...##.#\n\
                 .....#.#..",
                Some((Asteroid::new(6, 3), 41)),
            ),
            (
                ".#..##.###...#######\n\
                 ##.############..##.\n\
                 .#.######.########.#\n\
                 .###.#######.####.#.\n\
                 #####.##.#.##.###.##\n\
                 ..#####..#.#########\n\
                 ####################\n\
                 #.####....###.#.#.##\n\
                 ##.#################\n\
                 #####.##.###..####..\n\
                 ..######..##.#######\n\
                 ####.##.####...##..#\n\
                 .#####..#.######.###\n\
                 ##...#.##########...\n\
                 #.##########.#######\n\
                 .####.#.###.###.#.##\n\
                 ....##.##.###..#####\n\
                 .#.#.###########.###\n\
                 #.#.#.#####.####.###\n\
                 ###.##.####.##.#..##",
                Some((Asteroid::new(11, 13), 210)),
            ),
        ];
        for (input, expected) in &tests {
            let astromap = input.parse::<AstroMap>().unwrap();
            assert_eq!(part1(&astromap), *expected);
        }
    }
}
