use std::io;
use std::io::Read;
use std::fmt::Display;

use std::collections::HashSet;

type REPR = Vec<Card>;

pub struct Card {
    winning: Vec<i32>,
    have: Vec<i32>,
}

impl Card {
    fn num_matches(&self) -> u32 {
        let win_set = HashSet::<_>::from_iter(&self.winning);
        let have_set = HashSet::<_>::from_iter(&self.have);
        return win_set.intersection(&have_set).count().try_into().unwrap();
    }
}

pub fn compute_1(input: REPR) -> i32 {
    return input.into_iter()
    .map(|c| {
        let num_matches = c.num_matches();
        if c.num_matches() == 0 {0} else {2_i32.pow(num_matches - 1)}
    })
    .sum();
}

pub fn compute_2(input: REPR) -> i32 {
    let wins_per_card: Vec<_> = input.into_iter().map(|c| c.num_matches()).collect();
    let mut amounts_per_card = vec![1; wins_per_card.len()];
    for (processed_card, wins) in wins_per_card.iter().enumerate() {
        for win in 1..=*wins {
            let copied_card: usize = (processed_card as u32 + win) as usize;
            amounts_per_card[copied_card] += amounts_per_card[processed_card];
        }
    }
    return amounts_per_card.iter().sum();
}

pub fn parse(input: &str) -> REPR {
    return input.lines()
         .map(|l| {
            let (_, b) = l.split_once(": ").unwrap();
            let (win_str, have_str) = b.split_once(" | ").unwrap();
            let winning: Vec<i32> = win_str.split(" ").filter_map(|n| n.parse().ok()).collect();
            let have: Vec<i32> = have_str.split(" ").filter_map(|n| n.parse().ok()).collect();
            Card { winning, have }
            })
         .collect();
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute_2(parse(INPUT)), 30);
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