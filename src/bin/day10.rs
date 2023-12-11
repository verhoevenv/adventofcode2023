use std::convert::identity;
use std::fmt::{Display, Formatter, Write};
use std::io;
use std::io::Read;
use std::ops::{Index, IndexMut};

use crate::Direction::{East, North, South, West};
use crate::PipeDir::{EW, GROUND, NE, NS, NW, SE, START, SW};

type REPR = Pipes;
type Position = (usize, usize);

#[derive(Debug)]
pub struct Pipes {
    map: Arr2D<PipeDir>,
    start: Position,
}

impl Pipes {

    fn match_start(&self) -> PipeDir {
        let mut connecting_directions = Vec::with_capacity(2);
        for dir in [North, East, South, West] {
            if let Some(pos) = dir.apply(&self.start) {
                if self.map.is_valid_position(pos) {
                    let neighbour_pipe = self.map[pos];
                    if neighbour_pipe.other_side(&dir).is_ok() {
                        connecting_directions.push(dir);
                    }
                }
            }
        }

        assert_eq!(connecting_directions.len(), 2);

        match (connecting_directions[0], connecting_directions[1]) {
            (North, East) => NE,
            (North, South) => NS,
            (North, West) => NW,
            (East, South) => SE,
            (East, West) => EW,
            (South, West) => SW,
            (_, _) => panic!(),
        }
    }

    fn make_loop(&self, start_is: PipeDir) -> Vec<(Position, PipeDir)> {
        let mut result = vec![];
        let mut position = self.start;
        result.push((position, start_is));
        let mut going_to = start_is.connects()[0];

        loop {
            position = going_to.apply(&position).unwrap();

            if !self.map.is_valid_position(position) {
                panic!("The beast escaped! (pos = {:?})", position);
            }
            if self.map[position] == GROUND {
                panic!("The loop broke! (pos = {:?})", position);
            }
            if position == self.start {
                return result;
            }

            let pipe = self.map[position];
            result.push((position, pipe));
            going_to = pipe.other_side(&going_to).unwrap();
        }

    }
}

#[derive(Clone, Copy, Debug)]
enum Direction { North, East, South, West }

impl Direction {
    pub fn apply(&self, pos: &Position) -> Option<Position> {
        match self {
            North => if pos.0 == 0 { None } else { Some((pos.0 - 1, pos.1)) },
            East => Some((pos.0, pos.1 + 1)),
            South => Some((pos.0 + 1, pos.1)),
            West => if pos.1 == 0 { None } else { Some((pos.0, pos.1 - 1)) },
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PipeDir {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GROUND,
    START,
}

impl PipeDir {
    pub fn from_char(c: char) -> PipeDir {
        return match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => GROUND,
            'S' => START,
            _ => panic!("unknown map tile {}", c)
        }
    }

    pub fn connects(&self) -> [Direction; 2] {
        return match self {
            NS => [North, South],
            EW => [East, West],
            NE => [North, East],
            NW => [North, West],
            SW => [South, West],
            SE => [South, East],
            _ => panic!("no directions for {:?}", self),
        }
    }

    pub fn other_side(&self, incoming_dir: &Direction) -> Result<Direction, String> {
        return match (self, incoming_dir) {
            (NS, North) => Ok(North),
            (NS, South) => Ok(South),
            (EW, East) => Ok(East),
            (EW, West) => Ok(West),
            (NE, South) => Ok(East),
            (NE, West) => Ok(North),
            (NW, South) => Ok(West),
            (NW, East) => Ok(North),
            (SW, North) => Ok(West),
            (SW, East) => Ok(South),
            (SE, North) => Ok(East),
            (SE, West) => Ok(South),
            _ => Err(format!("no other side for {:?} -> {:?}", incoming_dir, self)),
        }
    }
}

pub fn compute_1(input: REPR) -> u64 {
    let start_pipe = input.match_start();
    let num_steps = input.make_loop(start_pipe).len();
    return (num_steps as u64) / 2;
}

pub fn compute_2(input: REPR) -> u64 {
    let start_pipe = input.match_start();
    let the_loop = input.make_loop(start_pipe);

    let mut high_res = Arr2D::new(' ', input.map.rows * 3, input.map.cols * 3);
    for ((x, y), pipe) in the_loop {
        let center = (x*3 + 1, y*3 + 1);
        high_res[(x*3 + 1, y*3 + 1)] = 'X';
        for dir in pipe.connects() {
            let side = dir.apply(&center).unwrap();
            high_res[side] = 'X';
        }
    }

    let starting_point = (0,0);
    high_res.flood_fill(starting_point, 'O', 'X');

    let mut num_inside = 0;
    for row in 0..input.map.rows {
        for col in 0..input.map.cols {
            if high_res[(row * 3 + 1, col * 3 + 1)] == ' ' {
                num_inside += 1;
            }
        }
    }

    return num_inside;
}

pub fn parse(input: &str) -> REPR {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let mut map = Arr2D::new(GROUND, rows, cols);
    let mut start = (0, 0);

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, col) in row.chars().enumerate() {
            map[(row_idx, col_idx)] = PipeDir::from_char(col);
            if col == 'S' {
                start = (row_idx, col_idx);
            }
        }
    }

    return Pipes {
        map,
        start
    };
}

#[derive(Debug)]
pub struct Arr2D<T> {
    rows: usize,
    cols: usize,
    arr: Vec<T>,
}

impl<T: Eq + Copy> Arr2D<T> {
    fn flood_fill(&mut self, start_point: (usize, usize), fill_with: T, wall: T) {
        let mut to_fill = vec![start_point];
        while let Some(pos) = to_fill.pop() {
            if self[pos] != wall && self[pos] != fill_with {
                self[pos] = fill_with;
                for neighbour in self.neighbours(pos).into_iter().filter_map(identity) {
                    to_fill.push(neighbour);
                }
            }
        }
    }
}

impl<T: Clone> Arr2D<T> {
    fn new(elem: T, rows: usize, cols: usize) -> Self {
        Arr2D {
            rows,
            cols,
            arr: vec![elem; rows*cols]
        }
    }

    fn is_valid_position(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.rows && pos.1 < self.cols
    }

    fn neighbours(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        [North, East, South, West]
            .map(|d| d.apply(&pos))
            .map(|d| d.filter(|&p| self.is_valid_position(p)))
    }
}

impl<T> Index<(usize, usize)> for Arr2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let row = index.0;
        let col = index.1;
        assert!(row < self.rows, "row out of bounds: {} should be smaller than {}", row, self.rows);
        assert!(col < self.cols, "col out of bounds: {} should be smaller than {}", col, self.cols);
        return &self.arr[row * self.cols + col];
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let row = index.0;
        let col = index.1;
        return &mut self.arr[row * self.cols + col];
    }
}

impl Display for Arr2D<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let x = self[(row, col)];
                f.write_char(x)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const INPUT1: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT1)), 8);
    }

    const INPUT2_1: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const INPUT2_2: &str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    const INPUT2_3: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_part2() {
        assert_eq!(compute_2(parse(INPUT2_1)), 4);
        assert_eq!(compute_2(parse(INPUT2_2)), 4);
        assert_eq!(compute_2(parse(INPUT2_3)), 10);
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