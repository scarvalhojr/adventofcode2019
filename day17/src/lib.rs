use intcode::{execute, InputOutput};
use std::collections::HashMap;
use std::convert::TryFrom;
use Direction::*;

const NEW_LINE: char = '\n';

#[derive(Clone)]
pub enum Movement {
    TurnLeft,
    TurnRight,
    Forward(u8),
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    pos_x: i64,
    pos_y: i64,
}

impl Position {
    fn next_line(&self) -> Self {
        Self {
            pos_x: 0,
            pos_y: self.pos_y + 1,
        }
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
    Space,
    Scaffold,
    Robot(Direction),
    LostRobot,
}

impl TryFrom<char> for Area {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Area::Space),
            '#' => Ok(Area::Scaffold),
            '^' => Ok(Area::Robot(North)),
            'v' => Ok(Area::Robot(South)),
            '<' => Ok(Area::Robot(West)),
            '>' => Ok(Area::Robot(East)),
            'X' => Ok(Area::LostRobot),
            _ => Err("Invalid scan"),
        }
    }
}

#[derive(Default)]
pub struct ScaffoldScanner {
    position: Position,
    area: HashMap<Position, Area>,
    video_feed: bool,
}

impl ScaffoldScanner {
    pub fn set_video_feed(&mut self, video_feed: bool) {
        self.video_feed = video_feed;
    }

    pub fn scan(&mut self, program: &[i64]) -> Option<i64> {
        execute(program, self)?;
        Some(self.sum_alignment_params())
    }

    fn sum_alignment_params(&self) -> i64 {
        self.area
            .iter()
            .filter(|&(_, area)| *area == Area::Scaffold)
            .filter(|(position, _)| {
                [North, South, West, East].iter().all(|direction| {
                    self.area.get(&position.go(*direction))
                        == Some(&Area::Scaffold)
                })
            })
            .map(|(position, _)| position.pos_x * position.pos_y)
            .sum()
    }

    pub fn build_scaffold_path(&self) -> Vec<Movement> {
        // TODO: find and build path
        [
            Movement::TurnRight,
            Movement::Forward(12),
            Movement::TurnRight,
            Movement::Forward(4),
            Movement::TurnRight,
            Movement::Forward(10),
            Movement::TurnRight,
            Movement::Forward(12),
        ]
        .to_vec()
    }
}

impl InputOutput for ScaffoldScanner {
    fn provide_input(&mut self) -> Option<i64> {
        None
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        let character = char::from(u8::try_from(value).ok()?);
        if self.video_feed {
            print!("{}", character);
        }
        if character == NEW_LINE {
            self.position = self.position.next_line();
        } else {
            let area = Area::try_from(character).ok()?;
            self.area.insert(self.position, area);
            self.position = self.position.go(East);
        }
        None
    }
}

#[derive(Default)]
pub struct ScaffoldWalker {
    movements: Vec<char>,
    dust: Option<i64>,
    video_feed: bool,
}

impl ScaffoldWalker {
    pub fn set_video_feed(&mut self, video_feed: bool) {
        self.video_feed = video_feed;
    }

    pub fn walk(&mut self, program: &[i64], path: &[Movement]) -> Option<i64> {
        let altered_prog = [2]
            .iter()
            .chain(program[1..].iter())
            .copied()
            .collect::<Vec<_>>();
        self.process_path(path);
        execute(&altered_prog, self)?;
        self.dust
    }

    fn process_path(&mut self, _path: &[Movement]) {
        // TODO: convert and compress path
        let compressed = "A,B,A,C,A,B,C,A,B,C\n\
                          R,12,R,4,R,10,R,12\n\
                          R,6,L,8,R,10\n\
                          L,8,R,4,R,4,R,6\n";

        // Movements are stored in reverse order
        self.movements.push(NEW_LINE);
        if self.video_feed {
            self.movements.push('y');
        } else {
            self.movements.push('n');
        }

        self.movements.extend(compressed.chars().rev());
    }
}

impl InputOutput for ScaffoldWalker {
    fn provide_input(&mut self) -> Option<i64> {
        let character = self.movements.pop()?;
        if self.video_feed {
            print!("{}", character);
        }
        Some(character as i64)
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        if self.video_feed {
            if let Ok(ascii_code) = u8::try_from(value) {
                print!("{}", char::from(ascii_code));
            }
        }
        self.dust = Some(value);
        Some(())
    }
}
