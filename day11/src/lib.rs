mod intcode;

use intcode::{execute, InputOutput};
use std::collections::HashMap;
use std::convert::TryFrom;
use Color::*;
use Direction::*;
use ReadingState::*;
use Turn::*;

enum Turn {
    TurnLeft,
    TurnRight,
}

impl TryFrom<i64> for Turn {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TurnLeft),
            1 => Ok(TurnRight),
            _ => Err("Invalid turn"),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Up
    }
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Up, TurnLeft) => Left,
            (Left, TurnLeft) => Down,
            (Down, TurnLeft) => Right,
            (Right, TurnLeft) => Up,
            (Up, TurnRight) => Right,
            (Right, TurnRight) => Down,
            (Down, TurnRight) => Left,
            (Left, TurnRight) => Up,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    pos_x: i32,
    pos_y: i32,
}

impl Position {
    fn new(pos_x: i32, pos_y: i32) -> Self {
        Position { pos_x, pos_y }
    }

    fn move_forward(self, direction: Direction) -> Self {
        match direction {
            Up => Self {
                pos_x: self.pos_x,
                pos_y: self.pos_y - 1,
            },
            Down => Self {
                pos_x: self.pos_x,
                pos_y: self.pos_y + 1,
            },
            Left => Self {
                pos_x: self.pos_x - 1,
                pos_y: self.pos_y,
            },
            Right => Self {
                pos_x: self.pos_x + 1,
                pos_y: self.pos_y,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl From<Color> for i64 {
    fn from(color: Color) -> Self {
        match color {
            Black => 0,
            White => 1,
        }
    }
}

impl From<Color> for char {
    fn from(color: Color) -> Self {
        match color {
            Black => ' ',
            White => '#',
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Black),
            1 => Ok(White),
            _ => Err("Invalid color"),
        }
    }
}

enum ReadingState {
    ReadColor,
    ReadTurn,
}

impl Default for ReadingState {
    fn default() -> Self {
        ReadColor
    }
}

#[derive(Default)]
struct PaintRobot {
    direction: Direction,
    position: Position,
    next_read: ReadingState,
    panels: HashMap<Position, Color>,
}

impl PaintRobot {
    fn paint_current_panel(&mut self, color: Color) {
        self.panels
            .entry(self.position)
            .and_modify(|c| *c = color)
            .or_insert(color);
    }

    fn count_painted_panels(&self) -> usize {
        self.panels.len()
    }

    fn display_panels(&self) -> Option<String> {
        let min_x = self.panels.keys().map(|pos| pos.pos_x).min()?;
        let max_x = self.panels.keys().map(|pos| pos.pos_x).max()?;
        let min_y = self.panels.keys().map(|pos| pos.pos_y).min()?;
        let max_y = self.panels.keys().map(|pos| pos.pos_y).max()?;
        let display = (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .map(|x| match self.panels.get(&Position::new(x, y)) {
                        Some(color) => char::from(*color),
                        None => ' ',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        Some(display)
    }
}

impl InputOutput for PaintRobot {
    fn provide_input(&mut self) -> Option<i64> {
        let color = self.panels.get(&self.position).unwrap_or(&Black);
        Some(i64::from(*color))
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        match self.next_read {
            ReadColor => {
                let color = Color::try_from(value).ok()?;
                self.paint_current_panel(color);
                self.next_read = ReadTurn;
            }
            ReadTurn => {
                let turn = Turn::try_from(value).ok()?;
                self.direction = self.direction.turn(turn);
                self.position = self.position.move_forward(self.direction);
                self.next_read = ReadColor;
            }
        }
        Some(())
    }
}

pub fn part1(program: &[i64]) -> Option<usize> {
    let mut robot = PaintRobot::default();
    execute(program, &mut robot)?;
    Some(robot.count_painted_panels())
}

pub fn part2(program: &[i64]) -> Option<String> {
    let mut robot = PaintRobot::default();
    robot.paint_current_panel(White);
    execute(program, &mut robot)?;
    robot.display_panels()
}
