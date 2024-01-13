advent_of_code::solution!(4);

// Count the number of winning card in hand
fn count_winning(winning: Vec<&str>, hand: Vec<&str>) -> u32 {
    let mut counter = 0;
    winning.iter().for_each(|i| {
        hand.iter().for_each(|j| {
            if *i == *j {
                counter += 1;
            }
        });
    });
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let input_line: Vec<_> = line.split(": ").collect();
            let numbers: Vec<_> = input_line[1].split(" | ").collect();

            let winning: Vec<&str> = numbers[0].split_whitespace().collect();
            let hand: Vec<&str> = numbers[1].split_whitespace().collect();

            let count = count_winning(winning, hand);
            let points = match count {
                0 => 0,
                _ => 1 << count - 1, // Double by every winning card
            };

            points
        })
        .sum::<u32>()
        .try_into()
        .ok()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
