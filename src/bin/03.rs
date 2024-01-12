advent_of_code::solution!(3);

use regex::Regex;

fn found_symbol(segment: &str) -> bool {
    for c in segment.chars() {
        if !c.is_digit(10) && c != '.' {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?m)[0-9]+").unwrap();

    let lines = input.lines().collect::<Vec<_>>();
    let mut sum: u32 = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let line_above = if i == 0 { lines[i + 1] } else { lines[i - 1] };
        let line_below = if i == lines.len() - 1 {
            lines[i - 1]
        } else {
            lines[i + 1]
        };

        sum += regex
            .find_iter(line)
            .map(|result| {
                let mut start = result.start();
                let mut end = result.end();

                if start > 0 {
                    start = start - 1;
                }
                if end < line.len() {
                    end = end + 1;
                }

                let check_line = &line[start..end];
                let check_below = &line_below[start..end];
                let check_above = &line_above[start..end];

                if found_symbol(check_line)
                    || found_symbol(check_below)
                    || found_symbol(check_above)
                {
                    result.as_str().parse::<u32>().unwrap()
                } else {
                    0
                }
            })
            .sum::<u32>()
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?m)[0-9]+").unwrap();
    let regex_symbol = Regex::new(r"(?m)\*+").unwrap();

    let lines = input.lines().collect::<Vec<_>>();
    let length = lines.len() - 1;

    lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if (1..length).contains(&i) {
                // Iterate over the gears in line
                let mut line_pairs: Vec<_> = Vec::new();
                line_pairs.extend(
                    regex_symbol
                        .find_iter(line)
                        .map(|symbol| {
                            let mut start = symbol.start();
                            let mut end = symbol.end();

                            // Max range of 3 in each direction
                            if start > 2 {
                                start = start - 3;
                            }
                            if end + 2 < line.len() {
                                end = end + 3;
                            }

                            // Space around the symbol to check
                            let lines = [
                                &lines[i - 1][start..end], // row above
                                &lines[i][start..end],     // symbol row
                                &lines[i + 1][start..end], // row below
                            ];

                            // Get numbers surrounding the symbol
                            let mut result: Vec<u32> = Vec::new();
                            lines.iter().for_each(|line| {
                                result.extend(regex.find_iter(line).filter_map(|number| {
                                    let start = number.start();
                                    let end = number.end() - 1; // remove extra offset
                                    let valid_range = 2..5;

                                    // numbers in the valid range are adjascent to the symbol
                                    if valid_range.contains(&start) || valid_range.contains(&end) {
                                        Some(number.as_str().parse::<u32>().unwrap())
                                    } else {
                                        None
                                    }
                                }));
                            });
                            result
                        })
                        // Only multiply gears (2 part numbers)
                        .filter_map(|f| {
                            if f.len() > 1 {
                                Some(f.iter().product())
                            } else {
                                None
                            }
                        })
                        // A vector of products
                        .collect::<Vec<u32>>(),
                );
                // Return sum of products
                return Some(line_pairs.iter().sum::<u32>());
            } else {
                None
            }
        })
        .sum::<u32>() // final sum
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
