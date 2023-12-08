use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::fmt::Display;

use regex::Regex;

type REPR = (Instructions, Network);

pub struct Instructions(Vec<char>);


pub struct Network {
    nodes: HashMap<Element, (Element, Element)>
}

#[derive(PartialEq, Eq, Hash)]
pub struct Element(String);


pub fn compute_1(input: REPR) -> u64 {
    let mut steps = 0;
    let mut current_element = &Element("AAA".to_owned());
    let instructions = input.0.0;

    while *current_element != Element("ZZZ".to_owned()) {
        steps += &instructions.len();
        for instr in &instructions {
            let (l, r) = input.1.nodes.get(&current_element).unwrap();
            match instr {
                'L' => current_element = l,
                'R' => current_element = r,
                _ => panic!("unknown instruction {}", instr),
            }    
        }
    }

    return steps as u64;
}

pub fn compute_2(input: REPR) -> u64 {
    todo!();
}

pub fn parse(input: &str) -> REPR {
    let node_re: Regex = Regex::new(r"^([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)$").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let instructions: Vec<char> = lines[0].chars().collect();

    let nodes = lines[2..].iter()
        .map(|&l| {
            let caps = node_re.captures(l).unwrap();
            let name = caps[1].parse().unwrap();
            let left = caps[2].parse().unwrap();
            let right = caps[3].parse().unwrap();
            (Element(name), Element(left), Element(right))})
        .fold(HashMap::new(), |mut acc, e| {
            acc.insert(e.0, (e.1, e.2));
            acc
        });

    return (Instructions(instructions), Network {nodes});
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT)), 6);
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