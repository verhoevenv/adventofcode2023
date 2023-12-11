use std::io;
use std::io::Read;
use std::fmt::Display;

type REPR = Vec<Sequence>;

#[derive(Debug)]
pub struct Sequence(Vec<i64>);

impl Sequence {
    pub fn diffs(&self) -> Sequence {
        Sequence(self.0.windows(2).map(|w| w[1] - w[0]).collect())
    }

    pub fn is_zeroes(&self) -> bool {
        return self.0.iter().all(|e| *e == 0);
    }
}


pub fn compute_1(input: REPR) -> i64 {
    let mut sum = 0;
    for s in input {
        let mut diffs = vec![s];
        while !diffs.last().unwrap().is_zeroes() {
            diffs.push(diffs.last().unwrap().diffs());
        }

        let mut next_val = 0;
        for diff in diffs.into_iter().rev() {
            next_val += diff.0.last().unwrap();
        }

        sum += next_val;
    }
    
    return sum;
}

pub fn compute_2(input: REPR) -> i64 {
    todo!();
}

pub fn parse(input: &str) -> REPR {
    return input.lines()
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
        .map(|v| Sequence(v))
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
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT)), 114);
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