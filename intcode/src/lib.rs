use std::collections::HashMap;
use std::convert::TryFrom;
use std::mem::replace;

const ADD_OP: i64 = 1;
const MULT_OP: i64 = 2;
const IN_OP: i64 = 3;
const OUT_OP: i64 = 4;
const JIT_OP: i64 = 5;
const JIF_OP: i64 = 6;
const LT_OP: i64 = 7;
const EQ_OP: i64 = 8;
const BASE_OP: i64 = 9;
const HALT_OP: i64 = 99;

const POSITION_MODE: i64 = 0;
const IMMEDIATE_MODE: i64 = 1;
const RELATIVE_MODE: i64 = 2;

type Memory = HashMap<usize, i64>;

pub trait InputOutput {
    fn provide_input(&mut self) -> Option<i64>;
    fn take_output(&mut self, value: i64) -> Option<()>;
}

pub fn execute(program: &[i64], io: &mut dyn InputOutput) -> Option<()> {
    let mut base = 0;
    let mut instr_ptr = 0;
    let mut memory = program.iter().cloned().enumerate().collect::<Memory>();

    loop {
        match memory.get(&instr_ptr).map(|&i| parse_instr(i)) {
            Some((HALT_OP, _, _, _)) => break,
            Some((ADD_OP, mode1, mode2, mode3)) => {
                let val1 = get_value(&memory, instr_ptr + 1, base, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, base, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3, base, mode3)?;
                set_value(&mut memory, addr, val1 + val2);
                instr_ptr += 4;
            }
            Some((MULT_OP, mode1, mode2, mode3)) => {
                let val1 = get_value(&memory, instr_ptr + 1, base, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, base, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3, base, mode3)?;
                set_value(&mut memory, addr, val1 * val2);
                instr_ptr += 4;
            }
            Some((IN_OP, mode, _, _)) => {
                let addr = get_addr(&memory, instr_ptr + 1, base, mode)?;
                set_value(&mut memory, addr, io.provide_input()?);
                instr_ptr += 2;
            }
            Some((OUT_OP, mode, _, _)) => {
                let val = get_value(&memory, instr_ptr + 1, base, mode)?;
                io.take_output(val);
                instr_ptr += 2;
            }
            Some((JIT_OP, mode1, mode2, _)) => {
                let val1 = get_value(&memory, instr_ptr + 1, base, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, base, mode2)?;
                if val1 != 0 {
                    instr_ptr = usize::try_from(val2).ok()?;
                } else {
                    instr_ptr += 3;
                }
            }
            Some((JIF_OP, mode1, mode2, _)) => {
                let val1 = get_value(&memory, instr_ptr + 1, base, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, base, mode2)?;
                if val1 == 0 {
                    instr_ptr = usize::try_from(val2).ok()?;
                } else {
                    instr_ptr += 3;
                }
            }
            Some((LT_OP, mode1, mode2, mode3)) => {
                let val1 = get_value(&memory, instr_ptr + 1, base, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, base, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3, base, mode3)?;
                set_value(&mut memory, addr, i64::from(val1 < val2));
                instr_ptr += 4;
            }
            Some((EQ_OP, mode1, mode2, mode3)) => {
                let val1 = get_value(&memory, instr_ptr + 1, base, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, base, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3, base, mode3)?;
                set_value(&mut memory, addr, i64::from(val1 == val2));
                instr_ptr += 4;
            }
            Some((BASE_OP, mode, _, _)) => {
                base += get_value(&memory, instr_ptr + 1, base, mode)?;
                instr_ptr += 2;
            }
            _ => {
                // Invalid instruction
                return None;
            }
        }
    }
    Some(())
}

fn parse_instr(instr: i64) -> (i64, i64, i64, i64) {
    let mut opcode = instr;
    let mode3 = opcode / 10_000;
    opcode %= 10_000;
    let mode2 = opcode / 1_000;
    opcode %= 1_000;
    let mode1 = opcode / 100;
    opcode %= 100;
    (opcode, mode1, mode2, mode3)
}

fn set_value(mem: &mut Memory, addr: usize, value: i64) {
    mem.entry(addr).and_modify(|v| *v = value).or_insert(value);
}

fn get_value(mem: &Memory, addr: usize, base: i64, mode: i64) -> Option<i64> {
    let pos = match mode {
        IMMEDIATE_MODE => addr,
        POSITION_MODE => usize::try_from(*mem.get(&addr)?).ok()?,
        RELATIVE_MODE => usize::try_from(base + *mem.get(&addr)?).ok()?,
        _ => return None,
    };
    // At this point, the memory position is an usize and hence valid;
    // if it's not yet set, return a Some(0)
    mem.get(&pos).cloned().or(Some(0))
}

fn get_addr(mem: &Memory, addr: usize, base: i64, mode: i64) -> Option<usize> {
    let value = match mode {
        POSITION_MODE => *mem.get(&addr)?,
        RELATIVE_MODE => base + *mem.get(&addr)?,
        _ => return None,
    };
    usize::try_from(value).ok()
}

#[derive(Default)]
pub struct SimpleInputOutput {
    input: Vec<i64>,
    output: Vec<i64>,
}

impl SimpleInputOutput {
    pub fn new(input_slice: &[i64]) -> Self {
        Self {
            input: input_slice.iter().rev().copied().collect(),
            output: Vec::new(),
        }
    }

    pub fn get_output(&mut self) -> Vec<i64> {
        replace(&mut self.output, Vec::new())
    }
}

impl InputOutput for SimpleInputOutput {
    fn provide_input(&mut self) -> Option<i64> {
        self.input.pop()
    }

    fn take_output(&mut self, value: i64) -> Option<()> {
        self.output.push(value);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let tests = vec![
            (
                vec![
                    109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101,
                    1006, 101, 0, 99,
                ],
                vec![
                    109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101,
                    1006, 101, 0, 99,
                ],
            ),
            (
                vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
                vec![1219070632396864],
            ),
            (vec![104, 1125899906842624, 99], vec![1125899906842624]),
        ];
        for (program, output) in tests {
            let mut test_io = SimpleInputOutput::default();
            assert_eq!(execute(&program, &mut test_io), Some(()));
            assert_eq!(test_io.output, output);
        }
    }

    #[test]
    fn test_parse_instr() {
        assert_eq!(parse_instr(0), (0, 0, 0, 0));
        assert_eq!(parse_instr(1), (1, 0, 0, 0));
        assert_eq!(parse_instr(102), (2, 1, 0, 0));
        assert_eq!(parse_instr(1003), (3, 0, 1, 0));
        assert_eq!(parse_instr(1104), (4, 1, 1, 0));
        assert_eq!(parse_instr(10005), (5, 0, 0, 1));
        assert_eq!(parse_instr(11006), (6, 0, 1, 1));
        assert_eq!(parse_instr(10107), (7, 1, 0, 1));
        assert_eq!(parse_instr(11199), (99, 1, 1, 1));
    }

    #[test]
    fn test_parse_instr_negative() {
        // Signal must be preserved so bad instructions are detected
        assert_eq!(parse_instr(-1), (-1, 0, 0, 0));
        assert_eq!(parse_instr(-102), (-2, -1, 0, 0));
        assert_eq!(parse_instr(-1003), (-3, 0, -1, 0));
        assert_eq!(parse_instr(-1104), (-4, -1, -1, 0));
        assert_eq!(parse_instr(-10005), (-5, 0, 0, -1));
        assert_eq!(parse_instr(-11006), (-6, 0, -1, -1));
        assert_eq!(parse_instr(-10107), (-7, -1, 0, -1));
        assert_eq!(parse_instr(-11199), (-99, -1, -1, -1));
    }
}
