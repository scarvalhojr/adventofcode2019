use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::str::FromStr;
use Direction::*;

type Position = (i16, i16);
type Portal = (char, char);
type AreaMap = HashMap<Position, Area>;

const MAZE_START: Portal = ('A', 'A');
const MAZE_END: Portal = ('Z', 'Z');

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn walk_from(&self, (pos_x, pos_y): Position) -> Position {
        match self {
            Up => (pos_x, pos_y - 1),
            Down => (pos_x, pos_y + 1),
            Left => (pos_x - 1, pos_y),
            Right => (pos_x + 1, pos_y),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Area {
    Start,
    Open,
    Portal(Portal),
    End,
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Area::Start, Area::Start) => true,
            (Area::Open, Area::Open) => true,
            (Area::End, Area::End) => true,
            (Area::Portal((s1, s2)), Area::Portal((o1, o2))) => {
                (s1 == o1 && s2 == o2) || (s1 == o2 && s2 == o1)
            }
            _ => false,
        }
    }
}

pub struct Maze {
    area: AreaMap,
    min_x: i16,
    max_x: i16,
    min_y: i16,
    max_y: i16,
}

impl Maze {
    pub fn shortest_path(&self, recursive_spaces: bool) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let (&start, _) = self.area.iter().find(|(_, &a)| a == Area::Start)?;
        let position = [Up, Down, Left, Right]
            .iter()
            .map(|direction| direction.walk_from(start))
            .find(|pos| self.area.get(pos) == Some(&Area::Open))?;
        queue.push_back((0, position, 0));

        while let Some((level, position, distance)) = queue.pop_front() {
            for direction in [Up, Down, Left, Right].iter() {
                let mut next_level = level;
                let mut next_pos = direction.walk_from(position);
                let mut next_dist = distance;

                match self.area.get(&next_pos) {
                    Some(Area::Open) => {
                        next_dist += 1;
                    }
                    Some(Area::Portal(portal)) => {
                        if recursive_spaces {
                            if self.is_outter_portal(next_pos) {
                                if level == 0 {
                                    // Outter portals are closed on level zero
                                    continue;
                                }
                                next_level = level - 1;
                            } else {
                                next_level = level + 1;
                            }
                        }
                        next_pos = self.warp_from(next_pos, *portal)?;
                    }
                    Some(Area::End) => {
                        if level == 0 {
                            return Some(distance);
                        }
                    }
                    _ => {
                        // Invalid move
                        continue;
                    }
                }

                if visited.insert((next_level, next_pos)) {
                    queue.push_back((next_level, next_pos, next_dist));
                }
            }
        }

        None
    }

    fn warp_from(&self, pos: Position, portal: Portal) -> Option<Position> {
        self.area
            .iter()
            .find(|&(p, a)| *a == Area::Portal(portal) && *p != pos)
            .map(|(p, _)| *p)
    }

    fn is_outter_portal(&self, (pos_x, pos_y): Position) -> bool {
        pos_x < self.min_x
            || pos_x > self.max_x
            || pos_y < self.min_y
            || pos_y > self.max_y
    }

    fn parse_tiles(tiles: Vec<(Position, char)>) -> Result<AreaMap, String> {
        let mut area: AreaMap = tiles
            .iter()
            .filter(|(_, ch)| *ch == '.')
            .map(|(pos, _)| (*pos, Area::Open))
            .collect();

        let mut first = VecDeque::new();
        let mut second = HashMap::new();
        for &(pos, ch) in tiles.iter().filter(|(_, ch)| *ch != '.') {
            if [Up, Down, Left, Right]
                .iter()
                .any(|dir| area.get(&dir.walk_from(pos)) == Some(&Area::Open))
            {
                first.push_back((pos, ch));
            } else {
                second.insert(pos, ch);
            }
        }

        while let Some((pos, first_char)) = first.pop_front() {
            let second_char = [Up, Down, Left, Right]
                .iter()
                .find_map(|dir| second.remove(&dir.walk_from(pos)))
                .ok_or_else(|| format!("incomplete portal {}", first_char))?;
            area.insert(
                pos,
                match (first_char, second_char) {
                    MAZE_START => Area::Start,
                    MAZE_END => Area::End,
                    portal => Area::Portal(portal),
                },
            );
        }

        Ok(area)
    }
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<(Position, char)> = s
            .lines()
            .enumerate()
            .flat_map(|(line_num, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch != '#' && *ch != ' ')
                    .map(move |(col_num, ch)| {
                        i16::try_from(line_num)
                            .and_then(|pos_y| {
                                i16::try_from(col_num)
                                    .map(|pos_x| ((pos_x, pos_y), ch))
                            })
                            .map_err(|_| "maze too large".to_string())
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| err.to_string())?;

        let area = Self::parse_tiles(tiles)?;
        let min_x = area
            .iter()
            .filter(|&(_, a)| *a == Area::Open)
            .map(|((pos_x, _), _)| *pos_x)
            .min()
            .unwrap_or(0);
        let max_x = area
            .iter()
            .filter(|&(_, a)| *a == Area::Open)
            .map(|((pos_x, _), _)| *pos_x)
            .max()
            .unwrap_or(0);
        let min_y = area
            .iter()
            .filter(|&(_, a)| *a == Area::Open)
            .map(|((_, pos_y), _)| *pos_y)
            .min()
            .unwrap_or(0);
        let max_y = area
            .iter()
            .filter(|&(_, a)| *a == Area::Open)
            .map(|((_, pos_y), _)| *pos_y)
            .max()
            .unwrap_or(0);

        Ok(Self {
            area,
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}
