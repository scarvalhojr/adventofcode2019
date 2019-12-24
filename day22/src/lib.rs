mod deck;
mod lcf;

use deck::Deck;
use lcf::LCF;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Shuffle {
    Cut(i32),
    DealWithIncr(usize),
    DealNewStack,
}

impl Shuffle {
    // Using modular arithmetic to represent each shuffle operation as
    // a linear congruential function f(x) = a * x + b mod m, that calculates
    // the new position of a card x in a deck of size m (see
    // https://codeforces.com/blog/entry/72593)
    fn get_lcf(&self, deck_size: i128) -> LCF {
        match self {
            Self::Cut(cut) => LCF::new(1, -cut as i128, deck_size),
            Self::DealWithIncr(incr) => LCF::new(*incr as i128, 0, deck_size),
            Self::DealNewStack => LCF::new(-1, -1, deck_size),
        }
    }

    fn combine(shuffles: &[Shuffle], deck_size: i128) -> LCF {
        shuffles
            .iter()
            .try_fold(LCF::identity(deck_size), |comb, shuffle| {
                comb.compose(&shuffle.get_lcf(deck_size))
            })
            .unwrap()
    }

    fn repeat(shuffles: &[Shuffle], times: u128, deck_size: i128) -> LCF {
        Self::combine(shuffles, deck_size).repeat(times)
    }
}

pub fn part1(shuffles: &[Shuffle]) -> i128 {
    let deck_size = 10_007;
    let lcf = Shuffle::combine(shuffles, deck_size);
    lcf.apply(2019)
}

pub fn part1_brute_force(shuffles: &[Shuffle]) -> usize {
    let mut deck = Deck::new(10_007);
    for shuffle in shuffles {
        deck.shuffle(*shuffle);
    }
    deck.find_card(2019).unwrap()
}

pub fn part2(shuffles: &[Shuffle]) -> i128 {
    let deck_size = 119_315_717_514_047;
    let repeat = 101_741_582_076_661;
    let position = 2020;
    let lcf = Shuffle::repeat(shuffles, repeat, deck_size);
    lcf.inverse(position)
}

impl FromStr for Shuffle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(concat!(
            r"^(cut (?P<cut>-?\d+))",
            r"|(deal with increment (?P<incr>\d+))",
            r"|(?P<new>deal into new stack)$",
        ))
        .unwrap()
        .captures(s)
        .ok_or_else(|| "Invalid input")?;

        if let Some(cap) = captures.name("cut") {
            let cut = cap.as_str().parse().map_err(|_| "invalid cut")?;
            Ok(Shuffle::Cut(cut))
        } else if let Some(cap) = captures.name("incr") {
            let incr = cap.as_str().parse().map_err(|_| "invalid increment")?;
            Ok(Shuffle::DealWithIncr(incr))
        } else if captures.name("new").is_some() {
            Ok(Shuffle::DealNewStack)
        } else {
            Err("invalid shuffle operation".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_cards(lcf: LCF, deck_size: usize) -> Vec<usize> {
        let mut cards = vec![0; deck_size];
        for card in 0..deck_size {
            let pos = lcf.apply(card as i128) as usize;
            cards[pos] = card;
        }
        cards
    }

    #[test]
    fn test_deal_new_stack() {
        let deck_size = 10;
        let lcf = Shuffle::DealNewStack.get_lcf(deck_size as i128);

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 9 8 7 6 5 4 3 2 1 0
        assert_eq!(
            get_cards(lcf, deck_size),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        );
    }

    #[test]
    fn test_cut() {
        let deck_size = 10;
        let lcf = Shuffle::Cut(3).get_lcf(deck_size as i128);

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 3 4 5 6 7 8 9 0 1 2
        assert_eq!(
            get_cards(lcf, deck_size),
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
        );
    }

    #[test]
    fn test_negative_cut() {
        let deck_size = 10;
        let lcf = Shuffle::Cut(-4).get_lcf(deck_size as i128);

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 6 7 8 9 0 1 2 3 4 5
        assert_eq!(
            get_cards(lcf, deck_size),
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
        );
    }

    #[test]
    fn test_deal_with_incr() {
        let deck_size = 10;
        let lcf = Shuffle::DealWithIncr(3).get_lcf(deck_size as i128);

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 0 7 4 1 8 5 2 9 6 3
        assert_eq!(
            get_cards(lcf, deck_size),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        );
    }

    #[test]
    fn test_repeat_shuffles() {
        let shuffles = [
            Shuffle::DealWithIncr(5),
            Shuffle::DealNewStack,
            Shuffle::Cut(6),
            Shuffle::Cut(-2),
            Shuffle::DealNewStack,
            Shuffle::Cut(5),
            Shuffle::DealNewStack,
            Shuffle::Cut(-4),
            Shuffle::DealWithIncr(7),
            Shuffle::Cut(3),
            Shuffle::DealWithIncr(3),
            Shuffle::DealNewStack,
        ];
        let tests = [
            (11, 1, vec![9, 0, 2, 4, 6, 8, 10, 1, 3, 5, 7]),
            (11, 7, vec![10, 6, 2, 9, 5, 1, 8, 4, 0, 7, 3]),
            (11, 13, vec![8, 5, 2, 10, 7, 4, 1, 9, 6, 3, 0]),
            (11, 19, vec![1, 7, 2, 8, 3, 9, 4, 10, 5, 0, 6]),
            (11, 23, vec![8, 5, 2, 10, 7, 4, 1, 9, 6, 3, 0]),
            (
                17,
                1,
                vec![14, 3, 9, 15, 4, 10, 16, 5, 11, 0, 6, 12, 1, 7, 13, 2, 8],
            ),
            (
                17,
                5,
                vec![10, 0, 7, 14, 4, 11, 1, 8, 15, 5, 12, 2, 9, 16, 6, 13, 3],
            ),
            (
                17,
                11,
                vec![1, 6, 11, 16, 4, 9, 14, 2, 7, 12, 0, 5, 10, 15, 3, 8, 13],
            ),
            (
                17,
                29,
                vec![15, 8, 1, 11, 4, 14, 7, 0, 10, 3, 13, 6, 16, 9, 2, 12, 5],
            ),
            (
                17,
                37,
                vec![10, 0, 7, 14, 4, 11, 1, 8, 15, 5, 12, 2, 9, 16, 6, 13, 3],
            ),
        ];
        for (deck_size, repeat, expected) in tests.iter() {
            let lcf = Shuffle::repeat(&shuffles, *repeat, *deck_size as i128);
            assert_eq!(get_cards(lcf, *deck_size), *expected);
        }
    }

    #[test]
    fn test_samples() {
        let deck_size = 10;
        let samples = [
            (
                vec![
                    Shuffle::DealWithIncr(7),
                    Shuffle::DealNewStack,
                    Shuffle::DealNewStack,
                ],
                vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
            ),
            (
                vec![
                    Shuffle::Cut(6),
                    Shuffle::DealWithIncr(7),
                    Shuffle::DealNewStack,
                ],
                vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
            ),
            (
                vec![
                    Shuffle::DealWithIncr(7),
                    Shuffle::DealWithIncr(9),
                    Shuffle::Cut(-2),
                ],
                vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
            ),
            (
                vec![
                    Shuffle::DealNewStack,
                    Shuffle::Cut(-2),
                    Shuffle::DealWithIncr(7),
                    Shuffle::Cut(8),
                    Shuffle::Cut(-4),
                    Shuffle::DealWithIncr(7),
                    Shuffle::Cut(3),
                    Shuffle::DealWithIncr(9),
                    Shuffle::DealWithIncr(3),
                    Shuffle::Cut(-1),
                ],
                vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
            ),
        ];

        for (shuffles, expected) in samples.iter() {
            let lcf = Shuffle::combine(shuffles, deck_size as i128);
            assert_eq!(get_cards(lcf, deck_size), *expected);
        }
    }
}
