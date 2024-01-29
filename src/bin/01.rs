advent_of_code::solution!(1);

use regex::Regex;

fn letter_to_digit(letter: &str) -> &str {
    match letter {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => letter,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|char| char.is_numeric())
                .collect::<Vec<_>>();

            format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let regex_rev = Regex::new(r"([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    input
        .lines()
        .map(|line| {
            let line_rev = line.chars().rev().collect::<String>(); // reversed line

            let first = letter_to_digit(regex.captures(line).unwrap().get(0).unwrap().as_str());
            let last = letter_to_digit(
                regex_rev
                    .captures(&line_rev)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str(),
            );

            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum::<i32>()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(931));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
