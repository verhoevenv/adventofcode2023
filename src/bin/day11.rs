use std::fmt::Display;
use std::io;
use std::io::Read;

type REPR = Universe;

#[derive(Debug, Clone)]
pub struct Universe(Vec<Galaxy>);

impl Universe {
    pub fn inflate(&self, amount: i64) -> Self {
        let max_x = self.0.iter().map(|g| g.coordinates.x).max().unwrap();
        let max_y = self.0.iter().map(|g| g.coordinates.y).max().unwrap();

        let mut new_universe = self.clone();

        for x in 0..max_x {
            if self.0.iter().all(|g| g.coordinates.x != x) {
                for g_idx in 0..self.0.len() {
                    if self.0[g_idx].coordinates.x > x {
                        new_universe.0[g_idx].coordinates.x += amount - 1;
                    }
                }
            }
        }
        for y in 0..max_y {
            if self.0.iter().all(|g| g.coordinates.y != y) {
                for g_idx in 0..self.0.len() {
                    if self.0[g_idx].coordinates.y > y {
                        new_universe.0[g_idx].coordinates.y += amount - 1;
                    }
                }
            }
        }

        return new_universe;
    }

    pub fn sum_of_distances(&self) -> i64 {
        let mut sum = 0;
        for (g1_idx, g1) in self.0.iter().enumerate() {
            for g2 in &self.0[g1_idx..] {
                sum += g1.distance(g2);
            }
        }
        return sum;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XY { x: i64, y: i64 }

#[derive(Debug, Clone)]
pub struct Galaxy {
    coordinates: XY
}

impl Galaxy {
    pub fn distance(&self, other: &Galaxy) -> i64 {
        (other.coordinates.x - self.coordinates.x).abs() + (other.coordinates.y - self.coordinates.y).abs()
    }
}

pub fn compute_1(input: REPR) -> i64 {
    return input.inflate(2).sum_of_distances();
}

pub fn compute_2(input: REPR) -> i64 {
    return input.inflate(1_000_000).sum_of_distances();
}

pub fn parse(input: &str) -> REPR {
    let mut result = Vec::new();

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            match char {
                '.' => {},
                '#' => result.push(Galaxy {coordinates: XY{x: col_idx as i64, y: row_idx as i64}}),
                _ => panic!("Unknown char: {}", char)
            }
        }
    }

    return Universe(result);
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT)), 374);
    }

    #[test]
    fn test_part2() {
        let universe = parse(INPUT);

        assert_eq!(universe.inflate(10).sum_of_distances(), 1030);
        assert_eq!(universe.inflate(100).sum_of_distances(), 8410);
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