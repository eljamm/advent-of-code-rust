use itertools::Itertools;

advent_of_code::solution!(5);

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

fn map_items(input: u64, output_map: &str) -> u64 {
    let items: Vec<&str> = output_map.split('\n').collect();
    let mut result = input;

    items.iter().for_each(|item| {
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

        if input >= dst && input < dst + range {
            result = src + input - dst;
        }
    });

    result
}

fn process_seeds(index: usize, seed: u64, maps: &Vec<&str>) -> u64 {
    if index < maps.len() {
        let next_item = map_items(seed, maps[index]);
        process_seeds(index + 1, next_item, maps)
    } else {
        seed
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.split("\n\n").collect_vec();

    let seeds = get_seeds(&lines);
    let maps = get_maps(&lines);

    let min = seeds
        .iter()
        .map(|seed| process_seeds(0, *seed, &maps))
        .min();

    min
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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

        let result = map_items(13, seed_mapping);
        assert_eq!(result, 13);

        let result = map_items(50, seed_mapping);
        assert_eq!(result, 52);

        let result = map_items(54, seed_mapping);
        assert_eq!(result, 56);

        let result = map_items(98, seed_mapping);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_soil_to_fertilizer() {
        let soil_mapping = indoc! {
            "0 15 37
            37 52 2
            39 0 15"
        };

        let result = map_items(0, soil_mapping);
        assert_eq!(result, 39);

        let result = map_items(15, soil_mapping);
        assert_eq!(result, 0);

        let result = map_items(25, soil_mapping);
        assert_eq!(result, 10);

        let result = map_items(52, soil_mapping);
        assert_eq!(result, 37);

        let result = map_items(53, soil_mapping);
        assert_eq!(result, 38);
    }

    #[test]
    fn test_process_seeds() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let lines = input.split("\n\n").collect_vec();

        let seed = 55;
        let maps = get_maps(&lines);

        let result = process_seeds(0, seed, &maps);
        assert_eq!(result, 86);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
