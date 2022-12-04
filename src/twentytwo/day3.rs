use itertools::Itertools;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|line| line.to_string())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u64 {
    input.iter()
        .map(|element| element.split_at(element.len() / 2))
        .map(|(first, second)| first.chars()
            .filter(|elem| second.find(*elem).is_some())
            .next().unwrap())
        .map(|missing_char| get_value(missing_char)).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u64 {
    input.iter()
        .chunks(3).into_iter()
        .map(|group| {
            // We combine the group into a vec
            let grouped: Vec<&String> = group.collect();
            // We need to find the only character that is in all 3
            Iterator::chain('A'..='Z', 'a'..='z')
                .filter(|char| get_badge(&grouped, char))
                .next().expect("There is no common elements for a group!")
        })
        .map(get_value)
        .sum()
}

fn get_badge(grouped: &Vec<&String>, char: &char) -> bool {
    grouped.iter()
        .filter(|elem| elem.find(*char).is_some())
        .count() == 3
}

fn get_value(missing_char: char) -> u64 {
    match missing_char {
        'A'..='Z' => (missing_char as u64) - 38,
        'a'..='z' => (missing_char as u64) - 96,
        _ => panic!("Wtf! {}", missing_char)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::twentytwo::day3::{get_value, part2};

    fn get_input() -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ]
    }

    #[test]
    fn example() {
        let result: String = get_input().iter()
            .map(|element| element.split_at(element.len() / 2))
            .map(|(first, second)| first.chars()
                .filter(|elem| second.find(*elem).is_some())
                .next().unwrap())
            .collect();

        assert_eq!(result, "pLPvts")
    }

    #[test]
    fn test_value() {
        assert_eq!(get_value('p'), 16);
        assert_eq!(get_value('L'), 38);
        assert_eq!(get_value('P'), 42);
        assert_eq!(get_value('v'), 22);
        assert_eq!(get_value('t'), 20);
        assert_eq!(get_value('s'), 19);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_input()), 70);
    }
}
