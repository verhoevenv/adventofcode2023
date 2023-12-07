use std::cmp::Ordering;
use std::io;
use std::io::Read;
use std::fmt;
use std::fmt::Display;

use itertools::Itertools;

type REPR = Vec<(Hand, u64)>;

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    pub fn new(cards_str: &str) -> Hand {
        let cards = cards_str.chars().map(|c| Card{ card: c }).collect();
        let hand_type = Self::calc_hand_type(&cards);
        Hand {
            cards,
            hand_type
        }
    }

    fn calc_hand_type(cards: &Vec<Card>) -> HandType {
        let counts: Vec<usize> = cards.iter().counts().values().sorted().rev().copied().collect();
        
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
        let hand_str = self.cards.iter().map(|c| c.card).join("");
        write!(f, "({}, {:?})", hand_str, self.hand_type)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Card {
    card: char
}

impl Card {
    pub fn strength(&self) -> u64 {
        "23456789TJQKA".find(self.card).unwrap() as u64
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

pub fn compute_1(mut input: REPR) -> u64 {
    input.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    return input.iter()
            .enumerate()
            .map(|(rank, hand_bid)| ((rank as u64 + 1), hand_bid))
            .map(|(rank, (_hand, bid))| rank * bid)
            .sum()
}

pub fn compute_2(input: REPR) -> u64 {
    todo!();
}

pub fn parse(input: &str) -> REPR {
    input.lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(hand, bid)| (Hand::new(hand), bid.parse().unwrap()))
        .collect()
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
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
        assert_eq!(Hand::new("32T3K").hand_type, HandType::OnePair);
        assert_eq!(Hand::new("KK677").hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("KTJJT").hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("T55J5").hand_type, HandType::ThreeOfAKind);
        assert_eq!(Hand::new("QQQJA").hand_type, HandType::ThreeOfAKind);

        let mut to_sort = [Hand::new("QQQJA"), Hand::new("T55J5")];
        to_sort.sort();
        assert_eq!(to_sort, [Hand::new("T55J5"), Hand::new("QQQJA")]);

        assert_eq!(compute_1(parse(INPUT)), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute_2(parse(INPUT)), todo!());
    }
}

fn read_and_write<T, S: Display>(parse: fn (&str) -> T, compute: &[fn(T) -> S] ) {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    for f in compute {
        let result = f(parse(&input));
        println!("{}", result);    
    }
}