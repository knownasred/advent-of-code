use itertools::Itertools;
use nom::character::complete::{line_ending, u64 as parse_u64};
use nom::IResult;
use nom::multi::{many1, many_till};
use nom::sequence::tuple;

pub fn parse_entry(input: &str) -> IResult<&str, u64> {
    let (rest, (result, _)) = tuple((parse_u64, line_ending))(input)?;

    Ok((rest, result))
}

pub fn parse_group(input: &str) -> IResult<&str, Vec<u64>> {
    let (rest, (entry, _)) = many_till(parse_entry, line_ending)(input)?;
    Ok((rest, entry))
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u64>> {
    let (rest, re) = many1(parse_group)(input).expect("An error occurred during parsing!");

    re
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u64>]) -> u64 {
    input.iter()
        .map(|elem| elem.iter().sum())
        .max().unwrap_or(0)
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u64>]) -> u64 {
    let mut elements: Vec<u64> = input.iter()
        .map(|elem| elem.iter().sum())
        .collect_vec();

    elements.sort();
    elements.reverse();

    elements.iter()
        .take(3)
        .sum()
}