use itertools::Itertools;
use std::convert::TryFrom;

const ADD_OP: i32 = 1;
const MULT_OP: i32 = 2;
const IN_OP: i32 = 3;
const OUT_OP: i32 = 4;
const JIT_OP: i32 = 5;
const JIF_OP: i32 = 6;
const LT_OP: i32 = 7;
const EQ_OP: i32 = 8;
const HALT_OP: i32 = 99;

const POSITION_MODE: i32 = 0;
const IMMEDIATE_MODE: i32 = 1;

pub fn part1(program: &[i32]) -> Option<i32> {
    (0..=4)
        .permutations(5)
        .filter_map(|parameters| amplify(program, &parameters))
        .max()
}

fn amplify(program: &[i32], parameters: &[i32]) -> Option<i32> {
    let mut signal = 0;
    for param in parameters {
        let mut memory = program.to_vec();
        let input = [*param, signal];
        let (result, _) = execute(&mut memory, &input, &mut 0, false)?;
        signal = result;
    }
    Some(signal)
}

pub fn part2(program: &[i32]) -> Option<i32> {
    (5..=9)
        .permutations(5)
        .filter_map(|parameters| amplify_with_feedback(program, &parameters))
        .max()
}

fn amplify_with_feedback(program: &[i32], parameters: &[i32]) -> Option<i32> {
    let mut signal = 0;
    let mut amplifiers = Vec::new();
    for param in parameters {
        let mut memory = program.to_vec();
        let input = [*param, signal];
        let mut instr_ptr = 0;
        let (result, _) = execute(&mut memory, &input, &mut instr_ptr, true)?;
        signal = result;
        amplifiers.push((memory, instr_ptr));
    }
    loop {
        for (memory, instr_ptr) in amplifiers.iter_mut() {
            let (result, halted) = execute(memory, &[signal], instr_ptr, true)?;
            if halted {
                return Some(signal);
            }
            signal = result;
        }
    }
}

fn execute(
    memory: &mut Vec<i32>,
    input: &[i32],
    instr_ptr: &mut usize,
    break_on_output: bool,
) -> Option<(i32, bool)> {
    let mut input_iter = input.to_vec().into_iter();
    let mut output = 0;

    loop {
        match memory.get(*instr_ptr).map(|&i| parse_instr(i)) {
            Some((HALT_OP, _, _)) => break,
            Some((ADD_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, *instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, *instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, *instr_ptr + 3)?;
                *memory.get_mut(addr)? = val1 + val2;
                *instr_ptr += 4;
            }
            Some((MULT_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, *instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, *instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, *instr_ptr + 3)?;
                *memory.get_mut(addr)? = val1 * val2;
                *instr_ptr += 4;
            }
            Some((IN_OP, _, _)) => {
                let addr = get_addr(&memory, *instr_ptr + 1)?;
                *memory.get_mut(addr)? = input_iter.next()?;
                *instr_ptr += 2;
            }
            Some((OUT_OP, _, _)) => {
                let addr = get_addr(&memory, *instr_ptr + 1)?;
                output = *memory.get_mut(addr)?;
                *instr_ptr += 2;
                if break_on_output {
                    return Some((output, false));
                }
            }
            Some((JIT_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, *instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, *instr_ptr + 2, mode2)?;
                if val1 != 0 {
                    *instr_ptr = usize::try_from(val2).ok()?;
                } else {
                    *instr_ptr += 3;
                }
            }
            Some((JIF_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, *instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, *instr_ptr + 2, mode2)?;
                if val1 == 0 {
                    *instr_ptr = usize::try_from(val2).ok()?;
                } else {
                    *instr_ptr += 3;
                }
            }
            Some((LT_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, *instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, *instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, *instr_ptr + 3)?;
                *memory.get_mut(addr)? = i32::from(val1 < val2);
                *instr_ptr += 4;
            }
            Some((EQ_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, *instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, *instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, *instr_ptr + 3)?;
                *memory.get_mut(addr)? = i32::from(val1 == val2);
                *instr_ptr += 4;
            }
            _ => {
                // Invalid instruction
                return None;
            }
        }
    }
    Some((output, true))
}

fn parse_instr(instr: i32) -> (i32, i32, i32) {
    let mut opcode = instr;
    let mode2 = opcode / 1_000;
    opcode %= 1_000;
    let mode1 = opcode / 100;
    opcode %= 100;
    (opcode, mode1, mode2)
}

fn get_value(mem: &[i32], ptr: usize, mode: i32) -> Option<i32> {
    match mode {
        POSITION_MODE => {
            let pos = mem.get(ptr).and_then(|&p| usize::try_from(p).ok())?;
            mem.get(pos).cloned()
        }
        IMMEDIATE_MODE => mem.get(ptr).cloned(),
        _ => None,
    }
}

fn get_addr(mem: &[i32], ptr: usize) -> Option<usize> {
    mem.get(ptr).and_then(|&a| usize::try_from(a).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let tests = [
            (
                vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99,
                    0, 0,
                ],
                43210,
            ),
            (
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5,
                    23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0,
                ],
                54321,
            ),
            (
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31,
                    0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4,
                    31, 99, 0, 0, 0,
                ],
                65210,
            ),
        ];
        for (prog, result) in &tests {
            assert_eq!(part1(prog), Some(*result));
        }
    }

    #[test]
    fn samples_part2() {
        let tests = [
            (
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                    27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ],
                139629729,
            ),
            (
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5,
                    55, 1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54,
                    53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
                    1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
                ],
                18216,
            ),
        ];
        for (prog, result) in &tests {
            assert_eq!(part2(prog), Some(*result));
        }
    }

    #[test]
    fn test_parse_instr() {
        assert_eq!(parse_instr(0), (0, 0, 0));
        assert_eq!(parse_instr(1), (1, 0, 0));
        assert_eq!(parse_instr(102), (2, 1, 0));
        assert_eq!(parse_instr(1003), (3, 0, 1));
        assert_eq!(parse_instr(1104), (4, 1, 1));
        assert_eq!(parse_instr(1005), (5, 0, 1));
        assert_eq!(parse_instr(1199), (99, 1, 1));
    }

    #[test]
    fn test_parse_instr_negative() {
        // Signal must be preserved so bad instructions are detected
        assert_eq!(parse_instr(-99), (-99, 0, 0));
        assert_eq!(parse_instr(-101), (-1, -1, 0));
        assert_eq!(parse_instr(-1002), (-2, 0, -1));
        assert_eq!(parse_instr(-1102), (-2, -1, -1));
    }
}
