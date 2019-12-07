fn is_strictly_increasing(pass_str: &str) -> bool {
    pass_str.as_bytes().windows(2).all(|dig| dig[0] <= dig[1])
}

fn has_double_digits(pass_str: &str) -> bool {
    pass_str.as_bytes().windows(2).any(|dig| dig[0] == dig[1])
}

pub fn part1(range_start: u32, range_end: u32) -> usize {
    (range_start..=range_end)
        .map(|password| password.to_string())
        .filter(|digits| {
            is_strictly_increasing(digits) && has_double_digits(digits)
        })
        .count()
}

fn has_strict_double_digits(pass_str: &str) -> bool {
    let (has_double, count, _) = pass_str.as_bytes().iter().fold(
        (false, 0, 0_u8),
        |(has_double, count, prev), &digit| {
            if digit == prev {
                (has_double, count + 1, digit)
            } else {
                (has_double || count == 2, 1, digit)
            }
        },
    );
    has_double || count == 2
}

pub fn part2(range_start: u32, range_end: u32) -> usize {
    (range_start..=range_end)
        .map(|password| password.to_string())
        .filter(|digits| {
            is_strictly_increasing(digits) && has_strict_double_digits(digits)
        })
        .count()
}
