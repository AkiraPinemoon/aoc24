use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_19").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();

    let towels = lines.next().unwrap().split(", ").collect_vec();
    lines.next();
    let patterns = lines.collect_vec();

    let mut cache = HashMap::new();

    let possible = patterns
        .iter()
        .filter(|pattern| is_possible(pattern, &towels, &mut cache))
        .count();
    println!("{}", possible);
}

pub fn part_two() {
    let mut file = File::open("input_19").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();

    let towels = lines.next().unwrap().split(", ").collect_vec();
    lines.next();
    let patterns = lines.collect_vec();

    let mut cache = HashMap::new();

    let count: usize = patterns
        .iter()
        .map(|pattern| count_arrangements(pattern, &towels, &mut cache))
        .sum();

    println!("{}", count);
}

fn is_possible<'a>(
    pattern: &'a str,
    towels: &Vec<&str>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(possible) = cache.get(pattern) {
        *possible
    } else {
        let possible = pattern.len() == 0
            || towels
                .iter()
                .filter_map(|towel| pattern.strip_prefix(towel))
                .any(|remaining| is_possible(remaining, towels, cache));

        cache.insert(pattern, possible);

        possible
    }
}

fn count_arrangements<'a>(
    pattern: &'a str,
    towels: &Vec<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(pattern) {
        *count
    } else {
        let count = towels
            .iter()
            .filter_map(|towel| pattern.strip_prefix(towel))
            .map(|remaining| count_arrangements(remaining, towels, cache))
            .sum();

        cache.insert(pattern, count);

        count
    }
}
