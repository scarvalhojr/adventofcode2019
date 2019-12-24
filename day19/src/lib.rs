use intcode::{execute, SimpleInputOutput};
use std::convert::TryFrom;

pub struct BeamScanner<'a> {
    program: &'a [i64],
    program_io: SimpleInputOutput,
}

impl<'a> BeamScanner<'a> {
    pub fn new(program: &'a [i64]) -> Self {
        Self {
            program,
            program_io: SimpleInputOutput::default(),
        }
    }

    pub fn scan_and_count(&mut self, dim_x: i64, dim_y: i64) -> Option<i64> {
        let mut count = 0;
        let mut row_start = 0;
        let mut row_end = 1;
        for pos_y in 0..dim_y {
            // Find the start of the row
            let mut empty_row = true;
            let mut pos_x = row_start;
            while pos_x < dim_x {
                if self.is_pulled(pos_x, pos_y)? {
                    row_start = pos_x;
                    empty_row = false;
                    break;
                }
                pos_x += 1;
            }
            if empty_row {
                continue;
            }

            // Find the end of the row
            if row_end <= row_start {
                row_end = row_start + 1;
            }
            while row_end < dim_x && self.is_pulled(row_end, pos_y)? {
                row_end += 1;
            }

            count += row_end - row_start;
        }
        Some(count)
    }

    pub fn find_fit(&mut self, dim_x: i64, dim_y: i64) -> Option<i64> {
        let mut row_ends = Vec::new();
        let mut row_start = 0;
        let mut row_end = 1;
        for pos_y in 0.. {
            // Find the start of the row
            let mut empty_row = true;
            let mut pos_x = row_start;
            // TODO: find a tighter limit for pos_x
            while pos_x < row_start + dim_x {
                if self.is_pulled(pos_x, pos_y)? {
                    row_start = pos_x;
                    empty_row = false;
                    break;
                }
                pos_x += 1;
            }
            if empty_row {
                row_ends.push(0);
                continue;
            }

            // Check if block fits
            if let Ok(top_row) = usize::try_from(pos_y - dim_y + 1) {
                if let Some(top_end) = row_ends.get(top_row) {
                    if *top_end >= row_start + dim_x {
                        return Some(row_start * 10_000 + (pos_y - dim_y + 1));
                    }
                }
            }

            // Find the end of the row
            if row_end <= row_start {
                row_end = row_start + 1;
            }
            while self.is_pulled(row_end, pos_y)? {
                row_end += 1;
            }
            row_ends.push(row_end);
        }
        None
    }

    fn is_pulled(&mut self, pos_x: i64, pos_y: i64) -> Option<bool> {
        self.program_io.add_input(&[pos_x, pos_y]);
        execute(self.program, &mut self.program_io)?;
        match self.program_io.get_output().pop()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}
