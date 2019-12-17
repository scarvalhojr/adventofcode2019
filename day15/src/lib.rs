use intcode::{execute, InputOutput};
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::thread::sleep;
use std::time::Duration;
use Direction::*;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

impl From<Direction> for i64 {
    fn from(direction: Direction) -> Self {
        match direction {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    pos_x: i64,
    pos_y: i64,
}

impl Position {
    fn new(pos_x: i64, pos_y: i64) -> Self {
        Position { pos_x, pos_y }
    }

    fn go(&self, direction: Direction) -> Self {
        match direction {
            North => Self {
                pos_x: self.pos_x,
                pos_y: self.pos_y - 1,
            },
            South => Self {
                pos_x: self.pos_x,
                pos_y: self.pos_y + 1,
            },
            West => Self {
                pos_x: self.pos_x - 1,
                pos_y: self.pos_y,
            },
            East => Self {
                pos_x: self.pos_x + 1,
                pos_y: self.pos_y,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Area {
    Wall,
    Empty,
    OxygenSystem,
}

impl TryFrom<i64> for Area {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Area::Wall),
            1 => Ok(Area::Empty),
            2 => Ok(Area::OxygenSystem),
            _ => Err("Invalid area"),
        }
    }
}

impl From<Area> for char {
    fn from(area: Area) -> Self {
        match area {
            Area::Empty => '.',
            Area::Wall => '#',
            Area::OxygenSystem => 'O',
        }
    }
}

#[derive(Default)]
pub struct MappingDroid {
    origin: Position,
    area: HashMap<Position, Area>,
    path: Vec<Direction>,
    position: Position,
    frametime: u64,
    display: bool,
}

impl MappingDroid {
    pub fn set_display(&mut self, display: bool, frametime: u64) {
        self.display = display;
        self.frametime = frametime;
    }

    pub fn scan(&mut self, program: &[i64]) -> Option<()> {
        self.area.entry(self.origin).or_insert(Area::Empty);
        execute(program, self)
    }

    fn get_oxygen_system_position(&self) -> Option<Position> {
        self.area
            .iter()
            .find(|&(_, area)| *area == Area::OxygenSystem)
            .map(|(pos, _)| *pos)
    }

    pub fn distance_to_oxygen_system(&self) -> Option<usize> {
        let oxygen = self.get_oxygen_system_position()?;
        self.distance(self.origin, Some(oxygen))
    }

    pub fn time_to_fill_area_with_oxygen(&self) -> Option<usize> {
        let oxygen = self.get_oxygen_system_position()?;
        self.distance(oxygen, None)
    }

    fn distance(&self, from: Position, to: Option<Position>) -> Option<usize> {
        let mut max_distance = 0;
        let mut pending = VecDeque::new();
        let mut visited = HashSet::new();

        pending.push_back((0, from));
        visited.insert(from);

        while let Some((mut distance, position)) = pending.pop_front() {
            if to.map(|target| target == position).unwrap_or(false) {
                return Some(distance);
            }
            max_distance = distance;
            distance += 1;
            for &direction in &[North, South, West, East] {
                let next_position = position.go(direction);
                if visited.contains(&next_position) {
                    continue;
                }
                if let Some(&area) = self.area.get(&next_position) {
                    if area != Area::Wall {
                        pending.push_back((distance, next_position));
                        visited.insert(next_position);
                    }
                }
            }
        }

        if to.is_none() {
            Some(max_distance)
        } else {
            // Target not found
            None
        }
    }

    fn display(&self) {
        if !self.display {
            return;
        }
        let min_x = self.area.keys().map(|pos| pos.pos_x).min().unwrap_or(0);
        let max_x = self.area.keys().map(|pos| pos.pos_x).max().unwrap_or(0);
        let min_y = self.area.keys().map(|pos| pos.pos_y).min().unwrap_or(0);
        let max_y = self.area.keys().map(|pos| pos.pos_y).max().unwrap_or(0);
        let display = (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .map(|x| {
                        let pos = Position::new(x, y);
                        if pos == self.origin {
                            '@'
                        } else if pos == self.position {
                            'D'
                        } else {
                            match self.area.get(&pos) {
                                Some(area) => char::from(*area),
                                None => ' ',
                            }
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}[2J{}", 27 as char, display);
        sleep(Duration::from_millis(self.frametime));
    }
}

impl InputOutput for MappingDroid {
    fn provide_input(&mut self) -> Option<i64> {
        self.display();
        // Try to move from current position
        for &direction in &[North, South, West, East] {
            let next_pos = self.position.go(direction);
            if !self.area.contains_key(&next_pos) {
                self.position = next_pos;
                self.path.push(direction);
                return Some(i64::from(direction));
            }
        }

        // If not possible, backtrack
        if let Some(last_direction) = self.path.pop() {
            let back = last_direction.opposite();
            self.position = self.position.go(back);
            return Some(i64::from(back));
        }

        // Nowhere to go: end of scan
        Some(0)
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        let area = Area::try_from(value).ok()?;
        self.area.entry(self.position).or_insert(area);
        if area == Area::Wall {
            // Backtrack
            let last_direction = self.path.pop()?;
            self.position = self.position.go(last_direction.opposite());
        }
        Some(())
    }
}
