use std::io;
use std::io::Read;
use std::fmt::Display;

pub fn first_and_last_digit(s: &str) -> u32 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

static DIGITS: [(&str, u32); 18] = 
    [("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ];

pub fn first_and_last_digit_with_letters(s: &str) -> u32 {
    let first = DIGITS.iter()
        .filter_map(|(digit, val)| s.find(digit).map(|p| (p, *val)))
        .min_by_key(|x| x.0)
        .unwrap().1;
    let last = DIGITS.iter()
        .filter_map(|(digit, val)| s.rfind(digit).map(|p| (p, *val)))
        .max_by_key(|x| x.0)
        .unwrap().1;

    return first * 10 + last;
}

pub fn calibration_value2(list: Vec<String>) -> u32 {
    return list.iter()
               .map(|s| first_and_last_digit_with_letters(&s))
               .sum();
}

pub fn calibration_value(list: Vec<String>) -> u32 {
    return list.iter()
               .map(|s| first_and_last_digit(&s))
               .sum();
}

pub fn parse(input: &str) -> Vec<String> {
    return input.lines().map(String::from).collect();
}

fn main() {
    read_and_write(parse, calibration_value2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_part1() {
        assert_eq!(calibration_value(parse(INPUT1)), 142);
    }

    const INPUT2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_part2() {
        assert_eq!(calibration_value2(parse(INPUT2)), 281);
    }
}

fn read_and_write<T, S: Display>(parse: fn(&str) -> T, compute: fn(T) -> S ) {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");


    let result = compute(parse(&input));
    println!("{}", result);
}