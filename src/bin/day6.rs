use std::io;
use std::io::Read;
use std::fmt::Display;
use std::iter::zip;

type REPR = Vec<Race>;

pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn win_points(&self) -> (u64, u64) {
        let time = self.time as f64;
        let distance = self.distance as f64;
        //find x for which
        //x * (time - x) > distance
        //-x^2 + x*time - distance > 0
        //determinant: time^2 - 4*(-1)*(-distance)
        //solutions: (-time +/- sqrt(determinant))/(2 * (-1))
        let determinant = (time * time - 4.0 * distance) as f64;
        let sol_a = (-time + determinant.sqrt()) / (-2.0);
        let sol_b = (-time - determinant.sqrt()) / (-2.0);

        // sol_a and sol_b are the exact zero points,
        // so the time further to the extremes is too slow,
        // so round towards center point
        let sol_from = sol_a.ceil() as u64;
        let sol_to = sol_b.floor() as u64;

        // however we need to be strictly greater to beat the record,
        // so test whether we are at zero point & add if needed
        let from = if sol_from * (self.time - sol_from) == self.distance {sol_from + 1} else {sol_from};
        let to = if sol_to * (self.time - sol_to) == self.distance {sol_to - 1} else {sol_to};
        return (from, to);
    }

}

pub fn compute_1(input: REPR) -> u64 {
    return input.iter()
        .map(|r| r.win_points())
        .map(|(from, to)| to - from + 1)
        .product();
}

pub fn compute_2(input: REPR) -> u64 {
    todo!();
}

pub fn parse(input: &str) -> REPR {
    let times: Vec<u64> = input.lines().nth(0).unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distance: Vec<u64> = input.lines().nth(1).unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

    return zip(times, distance).map(|(time, distance)| Race{time, distance}).collect();
}

fn main() {
    read_and_write(parse, &[compute_1, compute_2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_win_points() {
        assert_eq!(Race{time: 7, distance: 9}.win_points(), (2, 5));
        assert_eq!(Race{time: 15, distance: 40}.win_points(), (4, 11));
        assert_eq!(Race{time: 30, distance: 200}.win_points(), (11, 19));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Race{time: 7, distance: 9}.win_points(), (2, 5));
        assert_eq!(compute_1(parse(INPUT)), 288);
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