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

fn parse_instr(instr: i32) -> (i32, i32, i32) {
    let mut opcode = instr;
    let mode2 = opcode / 1_000;
    opcode -= mode2 * 1_000;
    let mode1 = opcode / 100;
    opcode -= mode1 * 100;
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

fn execute(program: &[i32], input: i32) -> Option<i32> {
    let mut instr_ptr = 0;
    let mut memory = program.to_vec();
    let mut output = None;

    loop {
        match memory.get(instr_ptr).map(|&i| parse_instr(i)) {
            Some((HALT_OP, _, _)) => break,
            Some((ADD_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3)?;
                *memory.get_mut(addr)? = val1 + val2;
                instr_ptr += 4;
            }
            Some((MULT_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3)?;
                *memory.get_mut(addr)? = val1 * val2;
                instr_ptr += 4;
            }
            Some((IN_OP, _, _)) => {
                let addr = get_addr(&memory, instr_ptr + 1)?;
                *memory.get_mut(addr)? = input;
                instr_ptr += 2;
            }
            Some((OUT_OP, _, _)) => {
                let addr = get_addr(&memory, instr_ptr + 1)?;
                output = memory.get_mut(addr).cloned();
                instr_ptr += 2;
            }
            Some((JIT_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, mode2)?;
                if val1 != 0 {
                    instr_ptr = usize::try_from(val2).ok()?;
                } else {
                    instr_ptr += 3;
                }
            }
            Some((JIF_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, mode2)?;
                if val1 == 0 {
                    instr_ptr = usize::try_from(val2).ok()?;
                } else {
                    instr_ptr += 3;
                }
            }
            Some((LT_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3)?;
                *memory.get_mut(addr)? = i32::from(val1 < val2);
                instr_ptr += 4;
            }
            Some((EQ_OP, mode1, mode2)) => {
                let val1 = get_value(&memory, instr_ptr + 1, mode1)?;
                let val2 = get_value(&memory, instr_ptr + 2, mode2)?;
                let addr = get_addr(&memory, instr_ptr + 3)?;
                *memory.get_mut(addr)? = i32::from(val1 == val2);
                instr_ptr += 4;
            }
            _ => {
                // Invalid instruction
                return None;
            }
        }
    }
    output
}

pub fn part1(program: &[i32]) -> Option<i32> {
    execute(program, 1)
}

pub fn part2(program: &[i32]) -> Option<i32> {
    execute(program, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

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
