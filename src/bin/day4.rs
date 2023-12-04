use std::io;
use std::io::Read;
use std::fmt::Display;

use std::collections::HashSet;

type REPR = Vec<Card>;

pub struct Card {
    id: i32,
    winning: Vec<i32>,
    have: Vec<i32>,
}

pub fn compute_1(input: REPR) -> i32 {
    return input.into_iter()
    .map(|c| {
        let win_set = HashSet::<_>::from_iter(c.winning);
        let have_set = HashSet::<_>::from_iter(c.have);
        let num_matches: u32 = win_set.intersection(&have_set).count().try_into().unwrap();
        if num_matches == 0 {0} else {2_i32.pow(num_matches - 1)}
    })
    .sum();
}

pub fn compute_2(input: REPR) -> i32 {
    todo!();
}

pub fn parse(input: &str) -> REPR {
    return input.lines()
         .map(|l| {
            let (a, b) = l.split_once(": ").unwrap();
            let id: i32 = a["Card".len()..].trim().parse().unwrap();
            let (win_str, have_str) = b.split_once(" | ").unwrap();
            let winning: Vec<i32> = win_str.split(" ").filter_map(|n| n.parse().ok()).collect();
            let have: Vec<i32> = have_str.split(" ").filter_map(|n| n.parse().ok()).collect();
            Card { id, winning, have }
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