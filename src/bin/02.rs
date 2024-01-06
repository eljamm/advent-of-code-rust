advent_of_code::solution!(2);

use std::ops::{Index, IndexMut};

fn get_max_cubes(color: &str) -> i32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    }
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Index<&'_ str> for Cubes {
    type Output = i32;
    fn index(&self, s: &str) -> &i32 {
        match s {
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for Cubes {
    fn index_mut(&mut self, s: &str) -> &mut i32 {
        match s {
            "red" => &mut self.red,
            "green" => &mut self.green,
            "blue" => &mut self.blue,
            _ => panic!("unknown field: {}", s),
        }
    }
}

fn examine_cubes(cubes: Vec<Vec<&str>>) -> bool {
    for cube in cubes.iter() {
        let color = cube[1];
        let number = cube[0].parse::<i32>().unwrap();
        let max_number = get_max_cubes(color);

        // If the game is impossible, skip the rest of the cubes
        if number > max_number {
            return false;
        }
    }

    true
}

fn examine_game(sets: &Vec<&str>) -> bool {
    for set in sets {
        let cubes = set
            .split(", ")
            .map(|x| x.split(' ').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Compare examined cubes with maximum values
        let possible_game = examine_cubes(cubes);

        // If the game is impossible, skip its unchecked sets
        if possible_game == false {
            return false;
        }
    }

    true
}

fn examine_cubes_two(cubes: Vec<Vec<&str>>, min_cubes: &mut Cubes) -> Option<i32> {
    for cube in cubes.iter() {
        let color = cube[1];
        let number = cube[0].parse::<i32>().unwrap();

        // Determine the minimum number of cubes
        if number > min_cubes[color] {
            min_cubes[color] = number;
        }
    }

    None
}

fn examine_game_two(sets: &Vec<&str>) -> Option<i32> {
    let mut min_cubes = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in sets {
        let cubes = set
            .split(", ")
            .map(|x| x.split(' ').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Compare examined cubes with maximum values
        examine_cubes_two(cubes, &mut min_cubes);
    }

    // Calculate power for game
    let power = min_cubes["red"] * min_cubes["green"] * min_cubes["blue"];

    Some(power)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Game ID
    let mut index: u32 = 1;

    input
        .lines()
        .map(|game| {
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
            let possible_game = examine_game(sets);

            // Move on to the next game
            index += 1;

            if possible_game {
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

pub fn part_two(input: &str) -> Option<u32> {
    // Game ID
    let mut index: i32 = 1;

    input
        .lines()
        .map(|game| {
            // Extract sets from game
            let sets: &Vec<&str> = &game[(7 + index.checked_ilog10().unwrap_or(0) + 1)
                .try_into()
                .unwrap()..]
                .split("; ")
                .collect::<Vec<_>>();

            // Get the power for each set
            let power = examine_game_two(sets).unwrap();

            // Move on to the next game
            index += 1;

            power
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
