use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
enum MapDirection {
    Forward,
    Reverse,
}

fn get_seeds(lines: &[&str]) -> Vec<u64> {
    lines[0][7..]
        .split(' ')
        .map(|f| f.parse::<u64>().unwrap())
        .collect_vec()
}

fn get_maps<'a>(lines: &'a [&'a str]) -> Vec<&'a str> {
    lines[1..]
        .iter()
        .map(|f| f.split_once('\n').unwrap().1)
        .collect_vec()
}

fn get_reverse_maps<'a>(lines: &'a [&'a str]) -> Vec<&'a str> {
    lines[1..]
        .iter()
        .map(|f| f.split_once('\n').unwrap().1)
        .rev()
        .collect_vec()
}

/// Returns the start and end of the smallest map range
///
/// Example:
/// map = "60 56 37
///        56 93 4"
///
/// min_range = (0, 56)
fn get_min_range(input_map: &str) -> (u64, u64) {
    let limits: Vec<&str> = input_map.split('\n').collect();
    let mut min = (0, 0);

    for item in limits {
        let (src, _dst, range) = item
            .split(' ')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.parse::<u64>().unwrap())
                }
            })
            .collect_tuple()
            .unwrap_or((0, 0, 0));
        if src == 0 {
            min = (src, src + range);
            break;
        }
        if min == (0, 0) || src < min.1 {
            min = (0, src);
        }
    }

    min
}

/// Map input to the next or previous almanac map
///
fn map_items(input: u64, reference_map: &str, direction: &MapDirection) -> u64 {
    let items: Vec<&str> = reference_map.split('\n').collect();
    let mut result = input;

    for item in items {
        let (src, dst, range) = item
            .split(' ')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.parse::<u64>().unwrap())
                }
            })
            .collect_tuple()
            .unwrap_or((0, 0, 0));

        match direction {
            MapDirection::Forward => {
                if input >= dst && input < dst + range {
                    result = src + input - dst;
                    break;
                }
            }
            MapDirection::Reverse => {
                if input >= src && input < src + range {
                    result = input + dst - src;
                    break;
                }
            }
        }
    }

    result
}

/// Recursively map items depending on the direction:
/// - Forward: seed to location
/// - Reverse: location to seed
fn process_items(index: usize, next: u64, maps: &Vec<&str>, direction: &MapDirection) -> u64 {
    if index < maps.len() {
        let next_item = map_items(next, maps[index], direction);
        process_items(index + 1, next_item, maps, direction)
    } else {
        next
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split("\n\n").collect_vec();

    let seeds = get_seeds(&lines);
    let maps = get_maps(&lines);

    let min = seeds
        .iter()
        .map(|seed| process_items(0, *seed, &maps, &MapDirection::Forward))
        .min();

    min
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split("\n\n").collect_vec();

    let seeds = get_seeds(&lines);
    let maps = get_reverse_maps(&lines);

    let seed_ranges = seeds.chunks(2).collect_vec();
    let mut min_seed = 0;

    let (loc_start, loc_end) = get_min_range(maps[0]);

    for location in loc_start..loc_end {
        let mut found_min_location = false;
        let seed = process_items(0, location, &maps, &MapDirection::Reverse);
        for range in &seed_ranges {
            let seed_start = range[0];
            let seed_end = range[1] + seed_start;

            if seed >= seed_start && seed <= seed_end {
                // stop on first minimum seed
                found_min_location = true;
                break;
            }
        }
        if found_min_location {
            min_seed = seed;
            break;
        }
    }
    Some(min_seed)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_seed_to_soil() {
        let seed_mapping = indoc! {
            "50 98 2
            52 50 48"
        };

        let direction = MapDirection::Forward;

        let result = map_items(13, seed_mapping, &direction);
        assert_eq!(result, 13);

        let result = map_items(50, seed_mapping, &direction);
        assert_eq!(result, 52);
    }

    #[test]
    fn test_soil_to_fertilizer() {
        let soil_mapping = indoc! {
            "0 15 37
            37 52 2
            39 0 15"
        };

        let direction = MapDirection::Forward;

        let result = map_items(0, soil_mapping, &direction);
        assert_eq!(result, 39);

        let result = map_items(15, soil_mapping, &direction);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_process_seeds() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let lines = input.split("\n\n").collect_vec();

        let seed = 55;
        let maps = get_maps(&lines);

        let direction = MapDirection::Forward;

        let result = process_items(0, seed, &maps, &direction);
        assert_eq!(result, 86);
    }

    #[test]
    fn test_min_loc_range() {
        let loc_map = indoc! {
            "60 56 37
            56 93 4"
        };

        let result = Some(vec![get_min_range(loc_map)]);
        assert_eq!(result, Some(vec![(0, 56)]));
    }

    #[test]
    fn test_reverse_map() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let lines = input.split("\n\n").collect_vec();

        let maps = get_reverse_maps(&lines);

        let direction = MapDirection::Reverse;

        let result = process_items(0, 35, &maps, &direction);
        assert_eq!(result, 13);

        let result = process_items(0, 82, &maps, &direction);
        assert_eq!(result, 79);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82));
    }
}
