use intcode::{execute, SimpleInputOutput};

pub fn part1(program: &[i64]) -> Option<i64> {
    let mut prog_io = SimpleInputOutput::new(&[1]);
    execute(program, &mut prog_io);
    prog_io.get_output().iter().last().cloned()
}

pub fn part2(program: &[i64]) -> Option<i64> {
    let mut prog_io = SimpleInputOutput::new(&[2]);
    execute(program, &mut prog_io);
    prog_io.get_output().iter().last().cloned()
}
