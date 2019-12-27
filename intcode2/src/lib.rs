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

pub struct IntcodeComputer {
    base: i64,
    instr_ptr: usize,
    memory: Memory,
    blocking_io: bool,
}

pub trait InputOutput {
    fn provide_input(&mut self) -> Option<i64>;
    fn take_output(&mut self, value: i64) -> Option<()>;
}

impl IntcodeComputer {
    pub fn new(program: &[i64], blocking_io: bool) -> Self {
        Self {
            base: 0,
            instr_ptr: 0,
            memory: program.iter().cloned().enumerate().collect::<Memory>(),
            blocking_io,
        }
    }

    pub fn run(&mut self, io: &mut dyn InputOutput) -> Option<bool> {
        loop {
            match self.fetch_instr() {
                Some((HALT_OP, _, _, _)) => break,
                Some((ADD_OP, mode1, mode2, mode3)) => {
                    let val1 = self.get_value(self.instr_ptr + 1, mode1)?;
                    let val2 = self.get_value(self.instr_ptr + 2, mode2)?;
                    let addr = self.get_addr(self.instr_ptr + 3, mode3)?;
                    self.set_value(addr, val1 + val2);
                    self.instr_ptr += 4;
                }
                Some((MULT_OP, mode1, mode2, mode3)) => {
                    let val1 = self.get_value(self.instr_ptr + 1, mode1)?;
                    let val2 = self.get_value(self.instr_ptr + 2, mode2)?;
                    let addr = self.get_addr(self.instr_ptr + 3, mode3)?;
                    self.set_value(addr, val1 * val2);
                    self.instr_ptr += 4;
                }
                Some((IN_OP, mode, _, _)) => {
                    if let Some(value) = io.provide_input() {
                        let addr = self.get_addr(self.instr_ptr + 1, mode)?;
                        self.set_value(addr, value);
                        self.instr_ptr += 2;
                    } else if self.blocking_io {
                        return Some(false);
                    } else {
                        return None;
                    }
                }
                Some((OUT_OP, mode, _, _)) => {
                    let value = self.get_value(self.instr_ptr + 1, mode)?;
                    if io.take_output(value).is_some() {
                        self.instr_ptr += 2;
                    } else if self.blocking_io {
                        return Some(false);
                    } else {
                        return None;
                    }
                }
                Some((JIT_OP, mode1, mode2, _)) => {
                    let val1 = self.get_value(self.instr_ptr + 1, mode1)?;
                    let val2 = self.get_value(self.instr_ptr + 2, mode2)?;
                    if val1 != 0 {
                        self.instr_ptr = usize::try_from(val2).ok()?;
                    } else {
                        self.instr_ptr += 3;
                    }
                }
                Some((JIF_OP, mode1, mode2, _)) => {
                    let val1 = self.get_value(self.instr_ptr + 1, mode1)?;
                    let val2 = self.get_value(self.instr_ptr + 2, mode2)?;
                    if val1 == 0 {
                        self.instr_ptr = usize::try_from(val2).ok()?;
                    } else {
                        self.instr_ptr += 3;
                    }
                }
                Some((LT_OP, mode1, mode2, mode3)) => {
                    let val1 = self.get_value(self.instr_ptr + 1, mode1)?;
                    let val2 = self.get_value(self.instr_ptr + 2, mode2)?;
                    let addr = self.get_addr(self.instr_ptr + 3, mode3)?;
                    self.set_value(addr, i64::from(val1 < val2));
                    self.instr_ptr += 4;
                }
                Some((EQ_OP, mode1, mode2, mode3)) => {
                    let val1 = self.get_value(self.instr_ptr + 1, mode1)?;
                    let val2 = self.get_value(self.instr_ptr + 2, mode2)?;
                    let addr = self.get_addr(self.instr_ptr + 3, mode3)?;
                    self.set_value(addr, i64::from(val1 == val2));
                    self.instr_ptr += 4;
                }
                Some((BASE_OP, mode, _, _)) => {
                    self.base += self.get_value(self.instr_ptr + 1, mode)?;
                    self.instr_ptr += 2;
                }
                _ => {
                    // Invalid instruction
                    return None;
                }
            }
        }
        Some(true)
    }

    fn fetch_instr(&self) -> Option<(i64, i64, i64, i64)> {
        self.memory.get(&self.instr_ptr).map(|&instr| {
            let mut opcode = instr;
            let mode3 = opcode / 10_000;
            opcode %= 10_000;
            let mode2 = opcode / 1_000;
            opcode %= 1_000;
            let mode1 = opcode / 100;
            opcode %= 100;
            (opcode, mode1, mode2, mode3)
        })
    }

    fn set_value(&mut self, addr: usize, value: i64) {
        self.memory
            .entry(addr)
            .and_modify(|v| *v = value)
            .or_insert(value);
    }

    fn get_value(&self, addr: usize, mode: i64) -> Option<i64> {
        let pos = match mode {
            IMMEDIATE_MODE => addr,
            POSITION_MODE => usize::try_from(*self.memory.get(&addr)?).ok()?,
            RELATIVE_MODE => {
                usize::try_from(self.base + *self.memory.get(&addr)?).ok()?
            }
            _ => return None,
        };
        // At this point, the memory position is an usize and hence valid;
        // if it's not yet set, return a Some(0)
        self.memory.get(&pos).cloned().or(Some(0))
    }

    fn get_addr(&self, addr: usize, mode: i64) -> Option<usize> {
        let value = match mode {
            POSITION_MODE => *self.memory.get(&addr)?,
            RELATIVE_MODE => self.base + *self.memory.get(&addr)?,
            _ => return None,
        };
        usize::try_from(value).ok()
    }
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
            let mut computer = IntcodeComputer::new(&program, false);
            assert_eq!(computer.run(&mut test_io), Some(true));
            assert_eq!(test_io.output, output);
        }
    }
}
