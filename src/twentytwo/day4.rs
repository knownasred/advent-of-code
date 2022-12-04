use std::cmp::{max, min};
use itertools::Itertools;
use nom::bytes::complete::{tag};
use nom::character::complete::u64;
use nom::combinator::opt;
use nom::IResult;
use nom::multi::{many1};
use nom::sequence::tuple;

#[derive(Debug, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn is_contained(&self, other: &Self) -> bool {
        return self.start >= other.start && self.end <= other.end;
    }
    pub fn overlap(&self, other: &Self) -> bool {
        let highest_start = max(self.start, other.start);
        let lowest_end = min(self.end, other.end);

        let count: i64 = lowest_end as i64 - highest_start as i64;
        return count >= 0;
    }
}

pub fn range_parser(input: &str) -> IResult<&str, Range> {
    let dash = tag("-");

    let (input, (start, _, end)) = tuple((u64, dash, u64))(input)?;

    Ok((input, Range {
        start,
        end,
    }))
}

pub fn line_generator(input: &str) -> IResult<&str, (Range, Range)> {
    let line_ending = tuple((opt(tag("\r")), tag("\n")));

    let (input, (tuple1, _, tuple2, _)) = tuple((
        range_parser,
        tag(","),
        range_parser,
        opt(line_ending)
    ))(input)?;

    Ok((input, (tuple1, tuple2)))
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Range, Range)> {
    let (_, groups) = many1(line_generator)(input).expect("An error occurred during parsing.");
    groups
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Range, Range)]) -> u64 {
    input.iter()
        .filter(|item|
            item.0.is_contained(&item.1) || item.1.is_contained(&item.0)
        )
        .count() as u64
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Range, Range)]) -> u64 {
    input.iter()
        .filter(|(range1, range2)| range1.overlap(range2))
        .count() as u64
}

#[cfg(test)]
mod tests {
    use crate::twentytwo::day4::{part1, part2};
    use crate::twentytwo::day4::{input_generator, Range};

    const STRING: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#;

    fn range(start: u64, end: u64) -> Range {
        Range {
            start,
            end,
        }
    }

    #[test]
    pub fn test_parser() {
        assert_eq!(input_generator("2-3,4-5\r\n5-7,7-9\r\n"), vec![
            (range(2, 3), range(4, 5)),
            (range(5, 7), range(7, 9)),
        ]);
    }

    #[test]
    pub fn test_part1() {
        let entry = input_generator(STRING);
        assert_eq!(part1(&entry), 2);
    }

    #[test]
    pub fn test_part2() {
        let entry = input_generator(STRING);
        assert_eq!(part2(&entry), 4);
    }
}