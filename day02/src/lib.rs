const ADD_OP: usize = 1;
const MULT_OP: usize = 2;
const HALT_OP: usize = 99;

fn execute(
    input: &[usize],
    param1: Option<usize>,
    param2: Option<usize>,
) -> Option<usize> {
    let mut instr_ptr = 0;
    let mut memory = input.to_vec();

    if let Some(val1) = param1 {
        *memory.get_mut(1)? = val1;
    }
    if let Some(val2) = param2 {
        *memory.get_mut(2)? = val2;
    }

    fn get_ops(mem: &[usize], iptr: usize) -> Option<(usize, usize, usize)> {
        let res_addr = mem.get(iptr + 3)?;
        let op1 = mem[iptr + 1];
        let op2 = mem[iptr + 2];
        let val1 = mem.get(op1)?;
        let val2 = mem.get(op2)?;
        Some((*res_addr, *val1, *val2))
    };

    loop {
        match memory.get(instr_ptr) {
            Some(&HALT_OP) => break,
            Some(&ADD_OP) => {
                let (res_addr, val1, val2) = get_ops(&memory, instr_ptr)?;
                *memory.get_mut(res_addr)? = val1 + val2;
                instr_ptr += 4;
            }
            Some(&MULT_OP) => {
                let (res_addr, val1, val2) = get_ops(&memory, instr_ptr)?;
                *memory.get_mut(res_addr)? = val1 * val2;
                instr_ptr += 4;
            }
            _ => {
                return None;
            }
        }
    }
    memory.get(0).cloned()
}

pub fn part1(input: &[usize]) -> Option<usize> {
    execute(input, Some(12), Some(2))
}

pub fn part2(input: &[usize]) -> Option<usize> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if let Some(19_690_720) = execute(input, Some(noun), Some(verb)) {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(execute(&[1, 0, 0, 0, 99], None, None), Some(2));
        assert_eq!(execute(&[2, 3, 0, 3, 99], None, None), Some(2));
        assert_eq!(execute(&[2, 4, 4, 5, 99, 0], None, None), Some(2));
        assert_eq!(
            execute(&[1, 1, 1, 4, 99, 5, 6, 0, 99], None, None),
            Some(30)
        );
        assert_eq!(
            execute(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], None, None),
            Some(3500)
        );
    }

    #[test]
    fn do_not_panic() {
        assert_eq!(execute(&[], None, None), None);
        assert_eq!(execute(&[99], None, None), Some(99));
        assert_eq!(execute(&[99], Some(0), None), None);
        assert_eq!(execute(&[99, 0], None, Some(0)), None);
        assert_eq!(execute(&[1], None, None), None);
        assert_eq!(execute(&[1, 0], None, None), None);
        assert_eq!(execute(&[1, 0, 0], None, None), None);
        assert_eq!(execute(&[1, 0, 0, 0], None, None), None);
        assert_eq!(execute(&[1, 5, 0, 0, 99], None, None), None);
        assert_eq!(execute(&[1, 0, 5, 0, 99], None, None), None);
        assert_eq!(execute(&[1, 0, 0, 5, 99], None, None), None);
        assert_eq!(execute(&[2], None, None), None);
        assert_eq!(execute(&[2, 0], None, None), None);
        assert_eq!(execute(&[2, 0, 0], None, None), None);
        assert_eq!(execute(&[2, 0, 0, 0], None, None), None);
        assert_eq!(execute(&[2, 5, 0, 0, 99], None, None), None);
        assert_eq!(execute(&[2, 0, 5, 0, 99], None, None), None);
        assert_eq!(execute(&[2, 0, 0, 5, 99], None, None), None);
    }
}
