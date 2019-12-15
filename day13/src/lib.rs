use intcode::{execute, InputOutput};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    pos_x: i64,
    pos_y: i64,
}

impl Position {
    fn new(pos_x: i64, pos_y: i64) -> Self {
        Position { pos_x, pos_y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err("Invalid color"),
        }
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => ' ',
            Tile::Wall => '🀫',
            Tile::Block => '#',
            Tile::Paddle => '=',
            Tile::Ball => 'o',
        }
    }
}

enum ReadingState {
    PosX,
    PosY,
    Tile,
}

impl Default for ReadingState {
    fn default() -> Self {
        ReadingState::PosX
    }
}

#[derive(Default)]
struct Game {
    tiles: HashMap<Position, Tile>,
    next_read: ReadingState,
    read_x: i64,
    read_y: i64,
    ball_x: i64,
    paddle_x: i64,
    score: i64,
    frametime: u64,
    display: bool,
}

impl Game {
    fn count_block_tiles(&self) -> usize {
        self.tiles.values().filter(|&t| *t == Tile::Block).count()
    }

    fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    fn set_frametime(&mut self, frametime: u64) {
        self.frametime = frametime;
    }

    fn display(&self) {
        if !self.display {
            return;
        }
        let min_x = self.tiles.keys().map(|pos| pos.pos_x).min().unwrap_or(0);
        let max_x = self.tiles.keys().map(|pos| pos.pos_x).max().unwrap_or(0);
        let min_y = self.tiles.keys().map(|pos| pos.pos_y).min().unwrap_or(0);
        let max_y = self.tiles.keys().map(|pos| pos.pos_y).max().unwrap_or(0);
        let display = (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .map(|x| match self.tiles.get(&Position::new(x, y)) {
                        Some(color) => char::from(*color),
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}[2J{}\nScore: {}", 27 as char, display, self.score);
        sleep(Duration::from_millis(self.frametime));
    }
}

impl InputOutput for Game {
    fn provide_input(&mut self) -> Option<i64> {
        self.display();
        if self.paddle_x < self.ball_x {
            Some(1)
        } else if self.paddle_x > self.ball_x {
            Some(-1)
        } else {
            Some(0)
        }
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        match self.next_read {
            ReadingState::PosX => {
                self.read_x = value;
                self.next_read = ReadingState::PosY;
            }
            ReadingState::PosY => {
                self.read_y = value;
                self.next_read = ReadingState::Tile;
            }
            ReadingState::Tile => {
                if self.read_x == -1 && self.read_y == 0 {
                    self.score = value;
                } else {
                    let pos = Position::new(self.read_x, self.read_y);
                    let tile = Tile::try_from(value).ok()?;
                    self.tiles.insert(pos, tile);
                    if tile == Tile::Ball {
                        self.ball_x = self.read_x;
                    } else if tile == Tile::Paddle {
                        self.paddle_x = self.read_x;
                    }
                }
                self.next_read = ReadingState::PosX;
            }
        }
        Some(())
    }
}

pub fn part1(program: &[i64]) -> Option<(usize)> {
    let mut game = Game::default();
    execute(program, &mut game)?;
    Some(game.count_block_tiles())
}

pub fn part2(program: &[i64], display: bool, frametime: u64) -> Option<i64> {
    let hacked_prog = [2]
        .iter()
        .chain(program[1..].iter())
        .copied()
        .collect::<Vec<_>>();
    let mut game = Game::default();
    game.set_display(display);
    game.set_frametime(frametime);
    execute(&hacked_prog, &mut game)?;
    game.display();
    Some(game.score)
}
