use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

#[derive(PartialEq, Eq, Debug)]
struct Card {
    id: u32,
    winners: HashSet<u32>,
    own_numbers: HashSet<u32>,
}


impl Card {
    fn from_line(line: &str) -> Card {
        let parts: Vec<&str> = line.split(": ").collect();
        let id: u32 = parts[0].split(" ").last().unwrap().parse().unwrap();
        let number_sets: Vec<&str> = parts[1].split(" | ").collect();
        let winners: HashSet<u32> = Self::extract_numbers(number_sets[0]);
        let own_numbers = Self::extract_numbers(number_sets[1]);

        Card {id, winners, own_numbers}
    }

    fn extract_numbers(numbers: &str) -> HashSet<u32> {
        numbers
            .split_whitespace()
            .map(|d| d.parse::<u32>().unwrap())
            .collect()
    }

    fn score(&self) -> u32 {
        let overlap = self.overlap();
        match overlap {
            0 => 0,
            _ => 1 << (overlap - 1)
        }
    }

    fn overlap(&self) -> u32 {
        self.own_numbers.intersection(&self.winners).count() as u32
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let cards = load_cards();

    let score: u32 = cards.iter()
        .map(|card| card.score())
        .sum();

    println!("Part 1: {score}");
}

fn part2() {
    let cards = load_cards();
    let mut card_counts: HashMap<u32, u64> = HashMap::new();
    cards.iter()
        .for_each(|card| {
            card_counts.insert(card.id, 1);
        });
    cards.iter()
        .for_each(|card| {
            let win_count = card.overlap();
            let self_count = card_counts.get(&card.id).unwrap().clone();

            let dup_range = Range {start: card.id + 1, end: card.id + win_count + 1};
            for duplicated_card in dup_range {
                let new_count = card_counts.get(&duplicated_card).unwrap().clone() + self_count;
                card_counts.insert(duplicated_card, new_count);
            }
        });

    let cards_total: u64 = card_counts.values().sum();

    println!("Part 2: {cards_total}");
}

fn load_cards() -> Vec<Card> {
    fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(Card::from_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::Card;

    #[test]
    fn card_stuff() {
        let test_card1 = Card::from_line("Card 1:  1 |  2  3  4");
        assert!(test_card1.winners.contains(&1));
        assert_eq!(test_card1.winners.len(), 1);
        assert!(test_card1.own_numbers.contains(&2));
        assert!(test_card1.own_numbers.contains(&3));
        assert!(test_card1.own_numbers.contains(&4));
        assert_eq!(test_card1.overlap(), 0);
        assert_eq!(test_card1.score(), 0);

        let test_card2 = Card::from_line("Card 2: 10 11 12 13 | 11 12 13 14");
        assert_eq!(test_card2.overlap(), 3);
        assert_eq!(test_card2.score(), 4);
    }
}
