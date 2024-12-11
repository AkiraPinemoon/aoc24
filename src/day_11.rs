use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part_one() {
    let mut file = File::open("input_11").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut stones = contents
        .split_whitespace()
        .map(|val| val.parse::<u64>().unwrap())
        .collect_vec();

    for _ in 0..25 {
        stones = blink(stones);
    }

    println!("{}", stones.len());
}

pub fn part_two() {
    let mut file = File::open("input_11").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut stones = contents
        .split_whitespace()
        .map(|val| val.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache = HashMap::new();
    let mut stats = Stats::new();

    let res = stones
        .iter()
        .map(|stone| blink_recursive(*stone, 75, &mut cache, &mut stats))
        .sum::<usize>();

    println!("{}", res);
    let calls = stats.cached + stats.calc;
    let percent_cached = stats.cached as f32 / (calls as f32 / 100.0);
    println!("calls: {}; percent cached {}%", calls, percent_cached);
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut res = Vec::new();

    for stone in stones {
        let stone_str = format!("{}", stone);
        if stone == 0 {
            res.push(1);
        } else if stone_str.len() % 2 == 0 {
            let (a, b) = stone_str.split_at(stone_str.len() / 2);
            res.push(a.parse().unwrap());
            res.push(b.parse().unwrap());
        } else {
            res.push(stone * 2024);
        }
    }

    res
}

fn blink_recursive(stone: u64, depth: u64, cache: &mut HashMap<(u64, u64), usize>, stats: &mut Stats) -> usize {
    if let Some(res) = cache.get(&(stone, depth)) {
        stats.cached += 1;
        return *res;
    }

    stats.calc += 1;

    if depth == 0 {
        return 1;
    }

    let stone_str = format!("{}", stone);
    if stone == 0 {
        let res = blink_recursive(1, depth - 1, cache, stats);
        cache.insert((stone, depth), res);
        return res;
    } else if stone_str.len() % 2 == 0 {
        let (a, b) = stone_str.split_at(stone_str.len() / 2);
        let res_a = blink_recursive(a.parse().unwrap(), depth - 1, cache, stats);
        let res_b = blink_recursive(b.parse().unwrap(), depth - 1, cache, stats);
        let res = res_a + res_b;
        cache.insert((stone, depth), res);
        return res;
    } else {
        let res = blink_recursive(stone * 2024, depth - 1, cache, stats);
        cache.insert((stone, depth), res);
        return res;
    }
}

struct Stats {
    pub cached: usize,
    pub calc: usize,
}

impl Stats {
    fn new() -> Self {
        Self { cached: 0, calc: 0 }
    }
}
