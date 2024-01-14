advent_of_code::solution!(4);

// Count the number of winning card in hand
fn count_matches(winning: Vec<&str>, hand: Vec<&str>) -> u32 {
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
            // Find where card numbers begin ':' and
            // where to separate left and right cards '|'
            let (card_index, number_index) = find_indexes(line);

            // Extract card information from line
            let cards_left: Vec<_> = line[card_index + 2..number_index - 1]
                .split_whitespace()
                .collect();
            let cards_right: Vec<_> = line[number_index + 2..].split_whitespace().collect();

            let win_count = count_matches(cards_left, cards_right);
            let points = match win_count {
                0 => 0,
                _ => 1 << win_count - 1, // Double by every winning card
            };

            points
        })
        .sum::<u32>()
        .try_into()
        .ok()
}

fn find_indexes(line: &str) -> (usize, usize) {
    let mut card_index = 0;
    let mut number_index = 0;

    for (i, c) in line.chars().enumerate() {
        if c == ':' {
            card_index = i;
        } else if c == '|' {
            number_index = i;
            break;
        }
    }
    (card_index, number_index)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let max_cards = lines.len();
    let mut scratchcards: Vec<u32> = vec![1; max_cards]; // Count original cards

    lines.iter().enumerate().for_each(|(card_id, line)| {
        // Find where card numbers begin ':' and
        // where to separate left and right cards '|'
        let (card_index, number_index) = find_indexes(line);

        // Extract card information from line
        let cards_left: Vec<_> = line[card_index + 2..number_index - 1]
            .split_whitespace()
            .collect();
        let cards_right: Vec<_> = line[number_index + 2..].split_whitespace().collect();

        // Count the number of next cards to copy
        let next_cards = count_matches(cards_left, cards_right);

        // Add copies to the next cards, counting previously copied cards
        for i in 1..next_cards + 1 {
            scratchcards[card_id + i as usize] += 1 * scratchcards[card_id]; // next_copies += 1 * current_copies;
        }
    });

    // Return the final number of cards
    Some(scratchcards.iter().sum::<u32>())
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
        assert_eq!(result, Some(30));
    }
}
