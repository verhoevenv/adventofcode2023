use std::io;
use std::io::Read;
use std::fmt::Display;

pub fn first_and_last_digit(s: &str) -> u32 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    return digits.first().unwrap() * 10 + digits.last().unwrap();
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
    read_and_write(parse, calibration_value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_part1() {
        assert_eq!(calibration_value(parse(INPUT)), 142);
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