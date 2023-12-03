use std::io;
use std::io::Read;
use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;


pub fn compute_1(input: Vec<Game>) -> i32 {
    fn is_possible(g: &Game) -> bool {
        return g.subsets.iter().all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14);
    }
    return input.into_iter().filter(is_possible).map(|g| g.id).sum();
}

pub fn compute_2(input: Vec<Game>) -> i32 {
    fn fewest_cubes(g: Game) -> Subset {
        let mut result = Subset { red: 0, green: 0, blue: 0 };
        for s in g.subsets {
            result.red = s.red.max(result.red);
            result.green = s.green.max(result.green);
            result.blue = s.blue.max(result.blue);
        }
        return result;
    }
    fn power(s: Subset) -> i32 {
        return s.red * s.blue * s.green;
    }
    return input.into_iter().map(fewest_cubes).map(power).sum();
}

pub struct Game {
    id: i32,
    subsets: Vec<Subset>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_re: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
        let caps = game_re.captures(s).unwrap();
        let id = caps[1].parse().unwrap();
        let subsets = caps[2].split("; ").map(|s| s.parse().unwrap()).collect();
        return Ok(Game {
            id,
            subsets
        });
    }
}

pub struct Subset {
    red: i32,
    green: i32,
    blue: i32,
}

impl FromStr for Subset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let splits = s.split(", ");
        for split in splits {
            let (num, color) = split.split_once(" ").unwrap();
            match color {
                "red" => red = num.parse().unwrap(),
                "green" => green = num.parse().unwrap(),
                "blue" => blue = num.parse().unwrap(),
                _ => panic!("Cannot match {}", color)
            }
        }
        return Ok(Subset {
            red, green, blue
        });
    }
}


pub fn parse(input: &str) -> Vec<Game> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.parse().unwrap());
    }
    return result;
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT)), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute_2(parse(INPUT)), 2286);
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