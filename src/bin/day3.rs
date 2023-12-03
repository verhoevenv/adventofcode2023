use std::io;
use std::io::Read;
use std::fmt::Display;

use std::ops::Index;
use std::ops::IndexMut;

type REPR = Arr2D<char>;

pub fn compute_1(input: REPR) -> i32 {
    fn is_symbol(c: char) -> bool {
        return !c.is_numeric() && c != '.';
    }
    let mut sum: u32 = 0;

    let mut row_idx = 0;
    while row_idx < input.rows {
        let mut col_idx = 0;
        while col_idx < input.cols {
            if input[(row_idx, col_idx)].is_numeric() {
                let (num, col_num_start, to) = parse_num(&input, (row_idx, col_idx));
                col_idx = to;
                
                let mut symbol_found = false;
                let scan_from = if col_num_start == 0 { 0 } else { col_num_start - 1 };
                let scan_to = if col_idx == input.cols { col_idx - 1 } else { col_idx };
                if row_idx != 0 {
                    for scan_col in scan_from..=scan_to {
                        if is_symbol(input[(row_idx - 1, scan_col)]) {
                            symbol_found = true;
                        }
                    }
                }
                if is_symbol(input[(row_idx, scan_from)]) {
                    symbol_found = true;
                }
                if is_symbol(input[(row_idx, scan_to)]) {
                    symbol_found = true;
                }
                if row_idx != input.rows - 1 {
                    for scan_col in scan_from..=scan_to {
                        if is_symbol(input[(row_idx + 1, scan_col)]) {
                            symbol_found = true;
                        }
                    }
                }

                if symbol_found {
                    sum = sum + num;
                }
            }
            col_idx += 1;
        }
        row_idx += 1;
    }

    return sum.try_into().unwrap();
}

pub fn compute_2(input: REPR) -> i32 {
    let mut sum: u32 = 0;

    let mut row_idx = 0;
    while row_idx < input.rows {
        let mut col_idx = 0;
        while col_idx < input.cols {
            if input[(row_idx, col_idx)] == '*' {
                let scan_from = if col_idx == 0 { 0 } else { col_idx - 1 };
                let scan_to = if col_idx == input.cols { col_idx } else { col_idx + 1};

                let mut nums_found = vec![];
                if row_idx != 0 {
                    let mut scan_col = scan_from;
                    while scan_col <= scan_to {
                        if input[(row_idx - 1, scan_col)].is_numeric() {
                            let (num, _, to) = parse_num(&input, (row_idx - 1, scan_col));
                            scan_col = to;
                            nums_found.push(num);
                        }
                        scan_col += 1;
                    }
                }
                if input[(row_idx, scan_from)].is_numeric() {
                    let (num, _, _) = parse_num(&input, (row_idx, scan_from));
                    nums_found.push(num);
                }
                if input[(row_idx, scan_to)].is_numeric() {
                    let (num, _, _) = parse_num(&input, (row_idx, scan_to));
                    nums_found.push(num);
                }
                if row_idx != input.rows - 1 {
                    let mut scan_col = scan_from;
                    while scan_col <= scan_to {
                        if input[(row_idx + 1, scan_col)].is_numeric() {
                            let (num, _, to) = parse_num(&input, (row_idx + 1, scan_col));
                            scan_col = to;
                            nums_found.push(num);
                        }
                        scan_col += 1;
                    }
                }

                if nums_found.len() == 2 {
                    sum = sum + (nums_found[0] * nums_found[1]);
                }
            }
            col_idx += 1;
        }
        row_idx += 1;
    }

    return sum.try_into().unwrap();
}

fn parse_num(arr: &Arr2D<char>, pos: (usize, usize)) -> (u32, usize, usize) {
    let row_idx = pos.0;
    let mut col_idx: usize = pos.1;

    while col_idx > 0 && arr[(row_idx, col_idx - 1)].is_numeric() {
        col_idx -= 1;
    }

    let col_num_start = col_idx;
    let mut num = arr[(row_idx, col_idx)].to_digit(10).unwrap();

    col_idx += 1;
    while col_idx < arr.cols && arr[(row_idx, col_idx)].is_numeric() {
        num *= 10;
        num += arr[(row_idx, col_idx)].to_digit(10).unwrap();
        col_idx += 1;
    }

    return (num, col_num_start, col_idx);
}

pub struct Arr2D<T> {
    rows: usize,
    cols: usize,
    arr: Vec<T>,
}

impl<T: Clone> Arr2D<T> {
    fn new(elem: T, rows: usize, cols: usize) -> Self {
        Arr2D { 
            rows,
            cols,
            arr: vec![elem; rows*cols]
         }
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

pub fn parse(input: &str) -> REPR {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let mut result = Arr2D::new(' ', rows, cols);
    
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, col) in row.chars().enumerate() {
            result[(row_idx, col_idx)] = col;
        }
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
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_part1() {
        assert_eq!(compute_1(parse(INPUT)), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute_2(parse(INPUT)), 467835);
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