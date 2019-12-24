use super::Shuffle;
use std::mem::replace;

#[derive(Clone)]
pub(super) struct Deck {
    cards: Vec<u32>,
    size: i32,
    start: i32,
    direct: i32,
}

impl Deck {
    pub(super) fn new(size: u32) -> Self {
        Self {
            cards: (0..size).collect(),
            size: size as i32,
            start: 0,
            direct: 1,
        }
    }

    pub(super) fn shuffle(&mut self, shuffle: Shuffle) {
        match shuffle {
            Shuffle::Cut(cut) => {
                self.start =
                    (self.size + self.start + cut * self.direct) % self.size;
            }
            Shuffle::DealWithIncr(incr) => {
                // TODO: ensure deck size is not a multiple of incr
                let mut cards = vec![0; self.cards.len()];
                let mut old_pos = self.start;
                let mut new_pos = 0;
                for _ in 0..self.size {
                    cards[new_pos] = self.cards[old_pos as usize];
                    new_pos = (new_pos + incr) % cards.len();
                    old_pos = (self.size + old_pos + self.direct) % self.size;
                }
                replace(&mut self.cards, cards);
                self.direct = 1;
                self.start = 0;
            }
            Shuffle::DealNewStack => {
                self.start = (self.start + self.size - self.direct) % self.size;
                self.direct *= -1;
            }
        }
    }

    pub(super) fn find_card(&self, card: u32) -> Option<usize> {
        let mut index = 0;
        let mut pos = self.start;
        while index < self.cards.len() {
            if self.cards[pos as usize] == card {
                return Some(index);
            }
            pos = (self.size + pos + self.direct) % self.size;
            index += 1;
        }
        None
    }

    #[cfg(test)]
    fn get_cards(&self) -> Vec<u32> {
        let mut cards = vec![0; self.cards.len()];
        let mut pos = self.start;
        for card in cards.iter_mut() {
            *card = self.cards[pos as usize];
            pos = (self.size + pos + self.direct) % self.size;
        }
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_new_stack() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::DealNewStack);

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 9 8 7 6 5 4 3 2 1 0
        assert_eq!(deck.get_cards(), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],);
    }

    #[test]
    fn test_cut() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(3));

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 3 4 5 6 7 8 9 0 1 2
        assert_eq!(deck.get_cards(), vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],);
    }

    #[test]
    fn test_negative_cut() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::Cut(-4));

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 6 7 8 9 0 1 2 3 4 5
        assert_eq!(deck.get_cards(), vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],);
    }

    #[test]
    fn test_deal_with_incr() {
        let mut deck = Deck::new(10);
        deck.shuffle(Shuffle::DealWithIncr(3));

        // Before: 0 1 2 3 4 5 6 7 8 9
        // After : 0 7 4 1 8 5 2 9 6 3
        assert_eq!(deck.get_cards(), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],);
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
            let mut deck = Deck::new(*deck_size);
            for _ in 0..*repeat {
                for shuffle in shuffles.iter() {
                    deck.shuffle(*shuffle);
                }
            }
            assert_eq!(deck.get_cards(), *expected);
        }
    }

    #[test]
    fn samples() {
        let tests = [
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

        for (shuffles, result) in &tests {
            let mut deck = Deck::new(10);
            for shuffle in shuffles {
                deck.shuffle(*shuffle);
            }
            assert_eq!(deck.get_cards(), *result);
        }
    }
}
