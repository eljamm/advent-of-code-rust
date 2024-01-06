advent_of_code::solution!(2);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    // Game ID
    let mut index: u32 = 1;

    input
        .lines()
        .map(|game| {
            let mut possible_game = true; // assume that games are possible by default

            // Extract sets from game
            // - Referencing the string slice is faster than splitting with ": "
            // - Ignoring the ID, 'Game ID: ' has 7 constant chars.
            // - To calculate the ID length, we use log10: id_length = log10(index) + 1
            let sets: &Vec<&str> = &game[(7 + index.checked_ilog10().unwrap_or(0) + 1)
                .try_into()
                .unwrap()..]
                .split("; ")
                .collect::<Vec<_>>();

            // Determine if the game is possible for each set
            for set in sets {
                let cubes = set
                    .split(", ")
                    .map(|x| x.split(' ').collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                // Compare examined cubes with maximum values
                for cube in cubes.iter() {
                    let color = cube[1];
                    let number = cube[0].parse::<i32>().unwrap();
                    let max_number = *max_cubes.get(color).unwrap();

                    // If the game is impossible, skip the rest of the cubes
                    if number > max_number {
                        possible_game = false;
                        break;
                    }
                }

                // If the game is impossible, skip its unchecked sets
                if possible_game == false {
                    break;
                }
            }

            // Move on to the next game
            index += 1;

            if possible_game == true {
                // We need to remove 1 since index starts from 1, not 0
                index - 1
            } else {
                0
            }
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
