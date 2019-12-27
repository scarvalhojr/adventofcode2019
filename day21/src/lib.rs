use intcode2::{InputOutput, IntcodeComputer};
use std::convert::TryFrom;

struct DroidInputOutput {
    video_feed: bool,
    script: Vec<char>,
    damage: i64,
}

impl DroidInputOutput {
    fn new(springscript: &str, video_feed: bool) -> Self {
        Self {
            video_feed,
            script: springscript.chars().rev().collect(),
            damage: 0,
        }
    }
}

impl InputOutput for DroidInputOutput {
    fn provide_input(&mut self) -> Option<i64> {
        let ch = self.script.pop()?;
        if self.video_feed {
            print!("{}", ch);
        }
        Some(ch as i64)
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        if let Ok(ascii) = u8::try_from(value) {
            if self.video_feed {
                print!("{}", char::from(ascii));
            }
        } else {
            self.damage = value;
        }
        Some(())
    }
}

fn run_script(program: &[i64], script: &str, video_feed: bool) -> Option<i64> {
    let mut droid_io = DroidInputOutput::new(script, video_feed);
    let mut droid = IntcodeComputer::new(program, true);
    droid.run(&mut droid_io)?;
    Some(droid_io.damage)
}

pub fn part1(program: &[i64], video_feed: bool) -> Option<i64> {
    // (NOT A) OR ((NOT B OR NOT C) AND D)
    let springscript = "NOT A J\n\
                        NOT B T\n\
                        AND D T\n\
                        OR T J\n\
                        NOT C T\n\
                        AND D T\n\
                        OR T J\n\
                        WALK\n";
    run_script(program, springscript, video_feed)
}

pub fn part2(program: &[i64], video_feed: bool) -> Option<i64> {
    // ((NOT B OR NOT C) AND D AND (E OR H)) OR (NOT A)
    //
    // Which is equivalent to:
    //
    // (NOT (B AND C) AND D AND (E OR H)) OR (NOT A)
    let springscript = "NOT B T\n\
                        NOT T T\n\
                        AND C T\n\
                        NOT T J\n\
                        AND D J\n\
                        NOT E T\n\
                        NOT T T\n\
                        OR H T\n\
                        AND T J\n\
                        NOT A T\n\
                        OR T J\n\
                        RUN\n";
    run_script(program, springscript, video_feed)
}
