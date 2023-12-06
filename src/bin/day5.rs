use std::io;
use std::convert::identity;
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
    seeds_as_ranges: Vec<Range>,
    maps: Vec<ConversionMap>,
}

impl Almanac {
    fn conversion_for(&self, step: &str) -> Option<&ConversionMap> {
        self.maps.iter().find(|&m| m.from == step)
    }

    fn convert<'a>(&'a self, t: TypeRange<'a>) -> TypeRange {
        let mut result = t;
        while let Some(conversion) = self.conversion_for(result.step) {
            result = conversion.map(result);
        }
        return result;
    }

    fn convert_seed(&self, seed_num: u64) -> u64 {
        let location = self.convert(TypeRange { step: "seed", ranges: vec![Range::new(seed_num, 1)] });
        assert_eq!(location.step, "location");
        assert_eq!(location.ranges.len(), 1);
        let location = &location.ranges[0];
        assert_eq!(location.length, 1);
        return location.start;
    }

    fn convert_seed_range(&self, seed_range: Range) -> Vec<Range> {
        let start = TypeRange { step: "seed", ranges: vec![seed_range] };
        let result = self.convert(start);
        return result.ranges;
    }
}

pub struct ConversionMap {
    from: String,
    to: String,
    ranges: Vec<RangeMap>,
}

impl ConversionMap {
    fn map(&self, input: TypeRange) -> TypeRange {
        assert!(input.step == self.from);

        let mut input_ranges = input.ranges.clone();
        let mut result_ranges = Vec::new();
        for map in &self.ranges {
            let mut unhandled_ranges = Vec::new();
            for range in input_ranges {
                let (handled, unhandled) = map.map(range);
                for r in handled {
                    result_ranges.push(r);
                }
                for r in unhandled {
                    unhandled_ranges.push(r);
                }    
            };
            input_ranges = unhandled_ranges;
        }
        for r in input_ranges {
            result_ranges.push(r);
        }
        
        TypeRange {
            step: &self.to,
            ranges: result_ranges
        }
    }
}

pub struct RangeMap {
    offset: i64,
    src: Range,
}

impl RangeMap {
    fn map(&self, range: Range) -> (Vec<Range>, Vec<Range>) {
        let (before, to_map, after) = self.src.split(range);

        let mapped = to_map.map(|r| Range::new(
            (r.start as i64 + self.offset) as u64,
            r.length
        ));

        let handled = vec![mapped].into_iter().filter_map(identity).collect();
        let unhandled = vec![before, after].into_iter().filter_map(identity).collect();
        return (handled, unhandled);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Range {
    start: u64,
    length: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, length: u64) -> Range {
        assert!(length > 0);
        Range {
            start,
            length,
            end: start + length
        }
    }

    fn end(&self) -> u64 {
        return self.start + self.length;
    }

    fn split(&self, to_split: Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let before = if to_split.start >= self.start {
             None
        } else {
            Some(Range::new(to_split.start, (self.start-to_split.start).min(to_split.length)))
        };

        let intersect_from = to_split.start.max(self.start);
        let intersect_to = to_split.end().min(self.end());
        let intersect = if intersect_from < intersect_to {
            Some(Range::new(intersect_from, intersect_to-intersect_from))
        } else {
            None
        };

        let after = if to_split.end() <= self.end() {
            None
        } else {
            let start = self.end().max(to_split.start);
            let length = to_split.end() - start;
            assert!(length > 0, "{:?} {:?}", self, to_split);
            Some(Range::new(start, length))
        };
       
        return (before, intersect, after);
    }
}

struct TypeRange<'a> {
    step: &'a str,
    ranges: Vec<Range>,
}

pub fn compute_1(input: REPR) -> u64 {
    input.seeds.iter()
    .map( |s| input.convert_seed(*s))
    .min().unwrap()
}

pub fn compute_2(input: REPR) -> u64 {
    input.seeds_as_ranges.iter()
    .flat_map( |r| input.convert_seed_range(*r))
    .map(|r| r.start)
    .min().unwrap()
}

pub fn parse(input: &str) -> REPR {
    fn range(i: &str) -> IResult<&str, RangeMap> {
        let (i, (dest_start, _, src_start, _,  length)) = tuple((
            num,
            tag(" "),
            num,
            tag(" "),
            num,
        ))(i)?;
        let offset = dest_start as i64 - src_start as i64;
        Ok((i, RangeMap { offset, src: Range::new(src_start, length) }))
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

        let seeds_as_ranges = seeds.chunks(2)
                                .map(|x| Range::new(x[0], x[1]))
                                .collect();

        Ok((i, Almanac { seeds, seeds_as_ranges, maps } ))
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
    fn test_split() {
        let r1 = Range::new(1, 10);
        let r2 = Range::new(2, 2);
        let r3 = Range::new(1, 1);
        let r4 = Range::new(4, 7);
        let r5 = Range::new(37, 7);
        assert_eq!(r1.split(r2), (None, Some(r2), None));
        assert_eq!(r2.split(r1), (Some(r3), Some(r2), Some(r4)));
        assert_eq!(r5.split(r1), (Some(r1), None, None));
        assert_eq!(r1.split(r5), (None, None, Some(r5)));
    }

    #[test]
    fn test_map() {
        let almanac = parse(INPUT);
        let map = almanac.conversion_for("seed").unwrap();
        let start_seed = Range::new(79, 1);
        let result = map.map(TypeRange { step: "seed", ranges: vec![start_seed] });

        assert_eq!(result.step, "soil");
        assert_eq!(result.ranges, vec![Range::new(81, 1)]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(compute_2(parse(INPUT)), 46);
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