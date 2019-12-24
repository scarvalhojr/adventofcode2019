use intcode2::{InputOutput, IntcodeComputer};
use std::collections::VecDeque;
use std::convert::TryFrom;
// use std::io::{self, BufRead};
use itertools::Itertools;

struct DroidInputOutput {
    buffer: VecDeque<char>,
    video_feed: bool,
}

impl DroidInputOutput {
    fn new(video_feed: bool) -> Self {
        Self {
            buffer: VecDeque::new(),
            video_feed,
        }
    }
}

impl InputOutput for DroidInputOutput {
    fn provide_input(&mut self) -> Option<i64> {
        // while self.buffer.is_empty() {
        //     let stdin = io::stdin();
        //     let line = stdin.lock().lines().next().unwrap().unwrap();
        //     self.buffer.extend(line.chars());
        //     self.buffer.push_back('\n');
        // }
        self.buffer.pop_front().map(|ch| ch as i64)
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        if let Ok(ascii) = u8::try_from(value) {
            if self.video_feed {
                print!("{}", char::from(ascii));
            }
        } else {
            panic!("Unexpected value: {}", value);
        }
        Some(())
    }
}

pub fn part1(program: &[i64], video_feed: bool) -> Option<i64> {
    let mut droid = IntcodeComputer::new(program, true);
    let mut droid_io = DroidInputOutput::new(video_feed);
    // This script will collect all (non-harmful) items and
    // bring the droid to the security checkpoint
    let script = "east\n\
                  take antenna\n\
                  east\n\
                  take ornament\n\
                  north\n\
                  west\n\
                  take fixed point\n\
                  east\n\
                  south\n\
                  west\n\
                  north\n\
                  west\n\
                  west\n\
                  take astronaut ice cream\n\
                  east\n\
                  south\n\
                  take hologram\n\
                  north\n\
                  east\n\
                  north\n\
                  take asterisk\n\
                  south\n\
                  south\n\
                  west\n\
                  south\n\
                  south\n\
                  south\n\
                  take dark matter\n\
                  north\n\
                  west\n\
                  north\n\
                  take monolith\n\
                  north\n\
                  north\n";
    droid_io.buffer.extend(script.chars());
    droid.run(&mut droid_io)?;
    let items = [
        "antenna",
        "ornament",
        "fixed point",
        "astronaut ice cream",
        "hologram",
        "asterisk",
        "dark matter",
        "monolith",
    ];
    for num_drops in 1..items.len() {
        for combination in items.iter().combinations(num_drops) {
            println!("Dropping {:?}", combination);
            for item in &combination {
                droid_io.buffer.extend("drop ".chars());
                droid_io.buffer.extend(item.chars());
                droid_io.buffer.push_back('\n');
            }
            droid_io.buffer.extend("east\n".chars());
            droid.run(&mut droid_io)?;
            for item in &combination {
                droid_io.buffer.extend("take ".chars());
                droid_io.buffer.extend(item.chars());
                droid_io.buffer.push_back('\n');
            }
        }
    }
    Some(0)
}
