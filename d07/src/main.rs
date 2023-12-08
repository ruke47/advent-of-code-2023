use std::cmp::Ordering;
use std::fs;
use itertools::Itertools;
use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, SinglePair, ThreeOfKind, TwoPair};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Hand {
    bid: u64,
    cards: Vec<Card>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard, SinglePair, TwoPair, ThreeOfKind, FullHouse, FourOfKind, FiveOfKind
}

impl Hand {
    fn rank_hand(&self) -> HandType {
        // group the cards by their identity
        let mut card_groups = self.cards.iter()
            .into_group_map_by(|c| *c);

        // set aside & count the jokers
        let joker_count = card_groups.remove(&Card::JOKER)
            .map(|jokers| jokers.len())
            .unwrap_or(0);

        // for each card group, count the size, then order the sizes high-to-low
        let mut card_sizes = card_groups.values()
            .map(|g| g.len())
            .sorted()
            .rev()
            .collect_vec();

        if card_sizes.is_empty() {
            // if we had all jokers, that's a 5 of a kind!
            card_sizes.push(joker_count);
        } else {
            // otherwise, add the jokers to whatever the biggest group was
            card_sizes[0] = card_sizes[0] + joker_count;
        }

        return if card_sizes[0] == 1 {
            HighCard
        } else if card_sizes[0] == 2 && card_sizes[1] == 1 {
            SinglePair
        } else if card_sizes[0] == 2 && card_sizes[1] == 2 {
            TwoPair
        } else if card_sizes.len() == 3 && card_sizes[0] == 3 {
            ThreeOfKind
        } else if card_sizes[0] == 3 && card_sizes[1] == 2 {
            FullHouse
        } else if card_sizes[0] == 4 {
            FourOfKind
        } else if card_sizes[0] == 5 {
            FiveOfKind
        } else {
            panic!("Not sure what kind of hand this is");
        }
    }

    fn from_string(line: &str) -> Hand {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cards: Vec<Card> = parts[0].chars()
            .map(|c| Card::from_char(c).unwrap())
            .collect();
        let bid: u64 = parts[1].parse().unwrap();

        Hand { bid, cards }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_matches = self.rank_hand();
        let b_matches = other.rank_hand();

        return if a_matches != b_matches {
            a_matches.cmp(&b_matches)
        } else {
            self.cards.iter()
                .zip(other.cards.iter())
                .find(|(ac, bc)| ac != bc)
                .map(|(ac, bc)| ac.cmp(bc))
                .unwrap_or(Ordering::Equal)
        };
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
enum Card {
    JOKER, C2, C3, C4, C5, C6, C7, C8, C9, CT, CJ, CQ, CK, CA,
}

#[derive(Debug, Eq, PartialEq)]
struct CardParseError;

impl Card {
    fn from_char(input: char) -> Result<Self, CardParseError> {
        match input {
            'j' => Ok(Card::JOKER),
            '2' => Ok(Card::C2),
            '3' => Ok(Card::C3),
            '4' => Ok(Card::C4),
            '5' => Ok(Card::C5),
            '6' => Ok(Card::C6),
            '7' => Ok(Card::C7),
            '8' => Ok(Card::C8),
            '9' => Ok(Card::C9),
            'T' => Ok(Card::CT),
            'J' => Ok(Card::CJ),
            'Q' => Ok(Card::CQ),
            'K' => Ok(Card::CK),
            'A' => Ok(Card::CA),
            __ => Err(CardParseError)
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut hands = read_hands("input", false);
    hands.sort();

    let sum_winnings: u64 = hands.iter().enumerate()
        .map(|(rank, hand)| hand.bid * ((rank + 1) as u64))
        .sum();

    println!("Part 1: {sum_winnings}");
}

fn part2() {
    let mut hands = read_hands("input", true);
    hands.sort();

    let sum_winnings: u64 = hands.iter().enumerate()
        .map(|(rank, hand)| hand.bid * ((rank + 1) as u64))
        .sum();

    println!("Part 2: {sum_winnings}");
}

fn read_hands(input_file: &str, jokers_trick: bool) -> Vec<Hand> {
    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            if jokers_trick {
                line.replace("J", "j")
            } else {
                String::from(line)
            }
        })
        .map(|line| Hand::from_string(&line))
        .collect()
}



#[cfg(test)]
mod tests {
    use crate::Card::{C4, CA, CT};
    use crate::{Hand, read_hands};

    #[test]
    fn line_to_hand() {
        let hand = Hand::from_string("AAT44 123");

        assert_eq!(hand.cards[0], CA);
        assert_eq!(hand.cards[1], CA);
        assert_eq!(hand.cards[2], CT);
        assert_eq!(hand.cards[3], C4);
        assert_eq!(hand.cards[4], C4);
        assert_eq!(hand.bid, 123);
    }

    #[test]
    fn sorts_by_hand() {
        let five_of_kind = Hand::from_string("44444 1");
        let four_of_kind = Hand::from_string("45444 1");
        let full_house = Hand::from_string("KK222 1");
        let three_of_kind = Hand::from_string("45464 1");
        let two_pair = Hand::from_string("45465 1");
        let single_pair = Hand::from_string("JK7J3 1");
        let nothing = Hand::from_string("JK723 1");

        let mut hands = vec![
            &full_house, &two_pair, &five_of_kind, &nothing,
            &three_of_kind, &single_pair, &four_of_kind
        ];

        hands.sort();
        assert_eq!(hands, vec![&nothing, &single_pair, &two_pair, &three_of_kind,
                               &full_house, &four_of_kind, &five_of_kind]);
    }

    #[test]
    fn sorts_by_card_order() {
        let c0 = Hand::from_string("257J3 1");
        let c1 = Hand::from_string("287J3 1");
        let c2 = Hand::from_string("2K7J3 1");
        let c3 = Hand::from_string("227J3 1");

        let mut hands = vec![
            &c2, &c0, &c1, &c3
        ];

        hands.sort();
        assert_eq!(hands, vec![&c0, &c1, &c2, &c3]);
    }

    #[test]
    fn part1_ex() {
        let mut hands = read_hands("example", false);
        hands.sort();

        let sum_winnings: u64 = hands.iter().enumerate()
            .map(|(rank, hand)| hand.bid * ((rank + 1) as u64))
            .sum();

        assert_eq!(sum_winnings, 6440);
    }

    #[test]
    fn part2_ex() {
        let mut hands = read_hands("example", true);
        hands.sort();

        let sum_winnings: u64 = hands.iter().enumerate()
            .map(|(rank, hand)| hand.bid * ((rank + 1) as u64))
            .sum();

        assert_eq!(sum_winnings, 5905);
    }
}
