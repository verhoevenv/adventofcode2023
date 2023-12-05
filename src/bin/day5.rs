use std::io;
use std::io::Read;
use std::fmt::Display;

use nom::IResult;
use nom::bytes::complete::{tag, take_while, take_until};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::tuple;

type REPR = Almanac;

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn conversion_for(&self, t: &TypeId) -> Option<&ConversionMap> {
        self.maps.iter().find(|&m| m.from == t.step)
    }

    fn convert<'a>(&'a self, t: TypeId<'a>) -> TypeId {
        let mut result = t;
        while let Some(conversion) = self.conversion_for(&result) {
            result = conversion.map(result);
        }
        return result;
    }

    fn convert_seed(&self, seed_num: u64) -> u64 {
        let location = self.convert(TypeId { step: "seed", num: seed_num });
        assert_eq!(location.step, "location");
        return location.num;
    }
}

pub struct ConversionMap {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl ConversionMap {
    fn map(&self, t: TypeId) -> TypeId {
        assert_eq!(self.from, t.step);

        let original = t.num;
        let mapped = self.ranges.iter().find_map(|r| r.map(original));

        let corresponding = mapped.unwrap_or(original);

        TypeId { 
            step: &self.to,
             num: corresponding
        }
    }
}

pub struct Range {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

impl Range {
    fn map(&self, num: u64) -> Option<u64> {
        if num >= self.src_start && num - self.src_start <= self.length  {
            return Some(num - self.src_start + self.dest_start);
        } else {
            return None;
        }
    }
}

struct TypeId<'a> {
    step: &'a str,
    num: u64,
}

pub fn compute_1(input: REPR) -> u64 {
    input.seeds.iter()
    .map( |s| input.convert_seed(*s))
    .min().unwrap()
}

pub fn compute_2(input: REPR) -> u64 {
    todo!();
}

pub fn parse(input: &str) -> REPR {
    fn range(i: &str) -> IResult<&str, Range> {
        let (i, (dest_start, _, src_start, _,  length)) = tuple((
            num,
            tag(" "),
            num,
            tag(" "),
            num,
        ))(i)?;
        Ok((i, Range { dest_start, src_start, length }))
    }

    fn conversion_map(i: &str) -> IResult<&str, ConversionMap> {
        let (i, from) = take_until("-to-")(i)?;
        let (i, _) = tag("-to-")(i)?;
        let (i, to) = take_until(" map:\n")(i)?;
        let (i, _) = tag(" map:\n")(i)?;
        let (i, ranges) = separated_list0(
            tag("\n"),
            range,
        )(i)?;
        Ok((i, ConversionMap { 
            from: from.to_owned(),
            to: to.to_owned(),
            ranges
         }))
    }

    fn num(i: &str) -> IResult<&str, u64> {
        map_res(
            take_while(|c: char| c.is_digit(10)),
            |n: &str| n.parse::<u64>()
        )(i)
    }

    fn seeds(i: &str) -> IResult<&str, Vec<u64>> {
        let (i, _) = tag("seeds: ")(i)?;
        let (i, result) = separated_list0(
            tag(" "),
            num,
        )(i)?;
        Ok((i, result))
    }

    fn all(i: &str) -> IResult<&str, Almanac> {
        let (i, seeds) = seeds(i)?;
        let (i, _) = tag("\n")(i)?;
        let (i, _) = tag("\n")(i)?;
        let (i, maps) = separated_list0(
            tag("\n\n"),
            conversion_map,
        )(i)?;
        let (i, _) = tag("\n")(i)?;

        Ok((i, Almanac { seeds, maps } ))
    }

    let (rest, result) = all(input).unwrap();
    assert!(rest.is_empty());
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
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_part1() {
        let almanac = parse(INPUT);

        assert_eq!(almanac.convert_seed(79), 82);
        assert_eq!(almanac.convert_seed(14), 43);
        assert_eq!(almanac.convert_seed(55), 86);
        assert_eq!(almanac.convert_seed(13), 35);

        assert_eq!(compute_1(parse(INPUT)), 35);
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