use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::fmt;

use itertools::Itertools;

type REPR = Vec<(Hand, u64)>;

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    pub fn new(cards_str: &str, with_joker: bool) -> Hand {
        let cards = cards_str.chars().map(|c| Card(c, with_joker)).collect();
        let hand_type = Self::calc_hand_type(&cards, with_joker);
        Hand {
            cards,
            hand_type
        }
    }

    fn calc_hand_type(cards: &Vec<Card>, with_joker: bool) -> HandType {
        let mut counts_per_card: HashMap<&Card, usize> = cards.iter().counts();

        let number_jokers = if with_joker {
            counts_per_card.remove(&Card('J', true)).unwrap_or(0)
        } else {
            0
        };
        
        let mut counts: Vec<_> = counts_per_card.values().sorted().rev().copied().collect();
        if counts.len() == 0 {
            counts.push(number_jokers);
        } else {
            counts[0] += number_jokers;
        }

        if counts.len() == 5 {
            return HandType::HighCard;
        }
        if counts.len() == 4 {
            return HandType::OnePair;
        }
        if counts.len() == 3 && counts[0] == 2 {
            return HandType::TwoPair;
        }
        if counts.len() == 3 && counts[0] == 3 {
            return HandType::ThreeOfAKind;
        }
        if counts.len() == 2 && counts[0] == 3 {
            return HandType::FullHouse;
        }
        if counts.len() == 2 && counts[0] == 4 {
            return HandType::FourOfAKind;
        }
        if counts.len() == 1 {
            return HandType::FiveOfAKind;
        }
        panic!("Unknown hand type for hand {:?} and counts {:?}", cards, &counts);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand_str = self.cards.iter().map(|c| c.0).join("");
        write!(f, "({}, {:?})", hand_str, self.hand_type)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Card(char, bool);

impl Card {
    pub fn strength(&self) -> u64 {
        if self.1 {
            "J23456789TQKA".find(self.0).unwrap() as u64
        } else {
            "23456789TJQKA".find(self.0).unwrap() as u64
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn compute(mut input: REPR) -> u64 {
    input.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    return input.iter()
            .enumerate()
            .map(|(rank, hand_bid)| ((rank as u64 + 1), hand_bid))
            .map(|(rank, (_hand, bid))| rank * bid)
            .sum()
}

pub fn parse1(input: &str) -> REPR {
    input.lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| (Hand::new(hand, false), bid.parse().unwrap()))
        .collect()
}

pub fn parse2(input: &str) -> REPR {
    input.lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| (Hand::new(hand, true), bid.parse().unwrap()))
        .collect()
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result1 = compute(parse1(&input));
    println!("{}", result1);    

    let result2 = compute(parse2(&input));
    println!("{}", result2);    
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_part1() {
        assert_eq!(Hand::new("32T3K", false).hand_type, HandType::OnePair);
        assert_eq!(Hand::new("KK677", false).hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("KTJJT", false).hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("T55J5", false).hand_type, HandType::ThreeOfAKind);
        assert_eq!(Hand::new("QQQJA", false).hand_type, HandType::ThreeOfAKind);

        let mut to_sort = [Hand::new("QQQJA", false), Hand::new("T55J5", false)];
        to_sort.sort();
        assert_eq!(to_sort, [Hand::new("T55J5", false), Hand::new("QQQJA", false)]);

        assert_eq!(compute(parse1(INPUT)), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute(parse2(INPUT)), 5905);
    }
}
