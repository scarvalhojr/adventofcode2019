use std::iter::successors;

pub fn part1(masses: &[u32]) -> u32 {
    masses.iter().map(|mass| (mass / 3).saturating_sub(2)).sum()
}

pub fn part2(masses: &[u32]) -> u32 {
    let fuel = |mass: &u32| (mass / 3).checked_sub(2);
    masses
        .iter()
        .map(|&mass| successors(fuel(&mass), fuel).sum::<u32>())
        .sum()
}
