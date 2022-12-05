use std::mem::transmute;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{anychar, line_ending, space0, u64 as parse_u64};
use nom::character::is_newline;

use nom::combinator::opt;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{delimited, terminated, tuple};
use crate::utils::transpose;

#[derive(Debug)]
pub struct Instruction {
    count: u64,
    from: u64,
    to: u64,
}

#[derive(Debug)]
pub struct Pile {
    contents: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

pub fn parse_entry(input: &str) -> IResult<&str, Option<char>> {
    if let (input, Some(_)) = opt(tag("   "))(input)? {
        // We need to remove a space if it is there:
        let (input, _) = opt(tag(" "))(input)?;
        Ok((input, None))
    } else {
        let (input, value) = delimited(tag("["), anychar, tag("]"))(input)?;
        // We need to remove a space if it is there:
        let (input, _) = opt(tag(" "))(input)?;
        Ok((input, Some(value)))
    }
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (reminder, (result, _)) = tuple((many1(parse_entry), line_ending))(input)?;
    Ok((reminder, result))
}

pub fn parse_contents(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    // We parse each line, and transmute afterwards:
    let (reminder, values) = many1(parse_line)(input)?;

    let result = transpose(values)
        .iter_mut().map(
        |column| {
            column.iter()
                .flat_map(|item| *item)
                .rev()
                .collect_vec()
        })
        .collect_vec();

    // We need to clear the line with whitespaces and the column numbers:
    let (reminder, _) = terminated(take_till(|c| c == '\n'), tag("\n"))(reminder)?;
    let (reminder, _) = terminated(take_till(|c| c == '\n'), tag("\n"))(reminder)?;
    Ok((reminder, result))
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (reminder, (
        _, count, _, from, _, to, _
    )) = tuple((
        tag("move "),
        parse_u64,
        tag(" from "),
        parse_u64,
        tag(" to "),
        parse_u64,
        opt(line_ending)
    ))(input)?;

    Ok((reminder, Instruction {
        count,
        from,
        to,
    }))
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(parse_instruction)(input)
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Pile {
    let (input, contents) = parse_contents(input)
        .expect("An exception occurred while parsing contents");
    let (input, instructions) = parse_instructions(input)
        .expect("An exception occurred while parsing instructions");

    assert_eq!(input.len(), 0,
               "There is still content in the input! {}", input);

    Pile {
        contents,
        instructions,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Pile) -> String {
    let mut contents = input.contents.clone();
    input.instructions.iter()
        .for_each(|instruction|
            (0..instruction.count)
                .for_each(|_| {
                    if let Some(value) = contents[(instruction.from - 1) as usize].pop() {
                        // Add it to the target
                        contents[(instruction.to - 1) as usize].push(value)
                    }
                })
        );

    println!("{:#?}", contents);

    contents.iter()
        .flat_map(|elem| elem.last())
        .collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &Pile) -> String {
    let mut contents = input.contents.clone();
    input.instructions.iter()
        .for_each(|instruction| {
            let len = contents[(instruction.from - 1) as usize].len();

            {
                contents[(instruction.from - 1) as usize]
                    .drain(len - instruction.count as usize..len)
                    .collect_vec()
            }.iter().for_each(|e| contents[(instruction.to - 1) as usize].push(*e));
        });

    contents.iter()
        .flat_map(|elem| elem.last())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::twentytwo::day5::{parse_input, part1, part2, Pile};

    pub fn input() -> Pile {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        parse_input(input)
    }

    #[test]
    fn test_input() {
        let input = input();

        assert_eq!(input.instructions.len(), 4);
        assert_eq!(input.contents[0].len(), 2);
        assert_eq!(input.contents[0], vec!['Z', 'N']);
        assert_eq!(input.instructions[0].count, 1);
        assert_eq!(input.instructions[0].from, 2);
        assert_eq!(input.instructions[0].to, 1);
    }

    #[test]
    fn test_part1() {
        let input = input();
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn test_part2() {
        let input = input();
        assert_eq!(part2(&input), "MCD");
    }
}