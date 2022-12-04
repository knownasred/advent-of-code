use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
pub enum Value {
    Rock,
    Paper,
    Scissors
}
#[derive(PartialEq, Debug)]
pub enum Outcome {
    Win,
    Draw,
    Loss
}

impl Outcome {
    pub fn points(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }

    pub fn get_value(&self, play: &Value) -> Value  {
        for value in [Value::Rock, Value::Paper, Value::Scissors] {
            if &value.wins(&play) == self {
                return value;
            }
        }
        unreachable!()
    }
}


impl Value {
    pub fn points(&self) -> u64 {
        match self {
            Value::Rock => 1,
            Value::Paper => 2,
            Value::Scissors => 3,
        }
    }


    pub fn wins(&self, other: &Self) -> Outcome {

        match self {
            Value::Rock => match other {
                Value::Rock => Outcome::Draw,
                Value::Paper => Outcome::Loss,
                Value::Scissors => Outcome::Win
            }
            Value::Paper =>  match other {
                Value::Rock => Outcome::Win,
                Value::Paper => Outcome::Draw,
                Value::Scissors => Outcome::Loss
            }
            Value::Scissors =>  match other {
                Value::Rock => Outcome::Loss,
                Value::Paper => Outcome::Win,
                Value::Scissors => Outcome::Draw
            }
        }
    }
}

pub struct Entry {
    pub input: Value,
    pub play: Value,
    pub outcome: Outcome
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    let mut entries = vec![];
    for line in input.lines() {
        // Get the first character
        entries.push(Entry {
            input: match line.chars().nth(0).unwrap() {
                'A' => Value::Rock,
                'B' => Value::Paper,
                'C' => Value::Scissors,
                _ => panic!("Unexpected input!")
            },
            play: match line.chars().nth(2).unwrap() {
                'X' => Value::Rock,
                'Y' => Value::Paper,
                'Z' => Value::Scissors,
                _ => panic!("Unexpected input!")
            },
            outcome: match line.chars().nth(2).unwrap() {
                'X' => Outcome::Loss,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => panic!("Unexpected input!")
            }
        });
    }

    entries
}

#[aoc(day2, part1)]
fn part1(input: &[Entry]) -> u64 {
    input.iter()
        .map(|entry| entry.play.points() + entry.play.wins(&entry.input).points())
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Entry]) -> u64 {
    input.iter()
        .map(|entry| entry.outcome.points() + entry.outcome.get_value(&entry.input).points())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::twentytwo::day2::{Outcome, Value};

    #[test]
    pub fn test_outcome() {
        assert_eq!(Outcome::Draw.get_value(&Value::Rock), Value::Rock);
        assert_eq!(Outcome::Loss.get_value(&Value::Paper), Value::Rock);

    }
}