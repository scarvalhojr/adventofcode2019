use std::char::from_digit;
use std::str::FromStr;

const SIGNATURE_LEN: usize = 8;

#[derive(Clone)]
pub struct FFT {
    digits: Vec<u8>,
    offset: usize,
}

struct Pattern {
    base: Vec<i32>,
    repeat: usize,
    index: usize,
}

impl FFT {
    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    fn pattern(index: usize) -> Pattern {
        let base = vec![0, 1, 0, -1];
        let repeat = index + 1;
        Pattern {
            base,
            repeat,
            index,
        }
    }

    fn next_phase(&mut self) {
        // The pattern for second half of the digits becomes
        // a series of 0s followed by 1s, which allows for a
        // much faster computation
        let fast_limit = self.digits.len() / 2;

        for index in self.offset..fast_limit {
            let sum = self.digits[index..]
                .iter()
                .zip(FFT::pattern(index))
                .fold(0_i32, |sum, (&digit, mult)| sum + digit as i32 * mult)
                .abs();
            self.digits[index] = (sum % 10) as u8;
        }

        // Once the pattern becomes 00...0011...11 the digits
        // can be computed by simply summing them back to front
        let mut sum = 0_u32;
        for index in (fast_limit..self.digits.len()).rev() {
            sum += self.digits[index] as u32;
            self.digits[index] = (sum % 10) as u8;
        }
    }

    fn signature(&self) -> String {
        self.digits[self.offset..self.offset + SIGNATURE_LEN]
            .iter()
            .map(|d| from_digit(*d as u32, 10))
            .collect::<Option<_>>()
            .unwrap()
    }

    fn replicate(&self, times: usize) -> Self {
        let mut digits = Vec::new();
        for _ in 0..times {
            digits.extend(self.digits.iter());
        }
        Self {
            digits,
            offset: self.offset,
        }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        Some(self.base[(self.index / self.repeat) % self.base.len()])
    }
}

pub fn part1(fft: &FFT) -> String {
    let mut result = fft.clone();
    for _ in 0..100 {
        result.next_phase();
    }
    result.signature()
}

pub fn part2(fft: &FFT) -> String {
    let offset = fft.digits[0..7]
        .iter()
        .fold(0_usize, |acc, &digit| 10 * acc + digit as usize);
    let mut result = fft.replicate(10_000);
    result.set_offset(offset);
    for _ in 0..100 {
        result.next_phase();
    }
    result.signature()
}

impl FromStr for FFT {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .map(|ch| ch.to_digit(10).map(|val| val as u8))
            .collect::<Option<_>>()
            .ok_or_else(|| "invalid input")?;
        Ok(Self { digits, offset: 0 })
    }
}
