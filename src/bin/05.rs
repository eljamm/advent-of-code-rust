use core::str;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, categories) = read_input(input).unwrap();
    eprintln!("DEBUGPRINT[2]: 05.rs:9: seeds={:#?}", seeds);
    eprintln!("DEBUGPRINT[3]: 05.rs:9: maps={:#?}", categories);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn read_input(input: &str) -> Option<(Vec<usize>, Vec<Category>)> {
    let lines = input.split("\n\n").collect_vec();

    let seeds = lines[0]
        .split(' ')
        .skip(1)
        .map(|f| f.parse::<usize>().unwrap())
        .collect_vec();

    let categories = lines[1..]
        .iter()
        .map(|c| c.split('\n').skip(1).map(split_map).collect::<Category>())
        .collect_vec();

    Some((seeds, categories))
}

fn split_map(input: &str) -> Map {
    let (dst, src, len) = input
        .split(' ')
        .map(|n| n.parse::<usize>().expect("Can't parse map values."))
        .collect_tuple()
        .unwrap();

    Map { dst, src, len }
}

fn map_backwards(map: &str, input: usize) -> Option<usize> {
    let Map { dst, src, len } = split_map(map);

    let dst_max = dst + len;
    let src_max = src + len;

    if (input <= dst_max) && (input >= dst) {
        return Some(src + (input - dst));
    }
    Some(input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn soil_to_seed() {
        let map = "50 98 2";
        let result = map_backwards(map, 51);
        assert_eq!(result, Some(99));
    }

    #[test]
    fn location_to_humidity() {
        let map = "56 93 4";
        let result = map_backwards(map, 35);
        assert_eq!(result, Some(35));
    }
}

#[derive(Default, Debug)]
struct Map {
    dst: usize,
    src: usize,
    len: usize,
}

impl Map {
    fn max_dst(self) -> usize {
        self.dst + self.len
    }
}

#[derive(Default, Debug)]
struct Category(Vec<Map>);

impl Category {
    fn new() -> Category {
        Category(Vec::with_capacity(7))
    }

    fn add(&mut self, elem: Map) {
        self.0.push(elem);
    }
}

impl FromIterator<Map> for Category {
    fn from_iter<T: IntoIterator<Item = Map>>(iter: T) -> Self {
        let mut c = Category::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
