use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_22").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let secrets = contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect_vec();

    let sum: u64 = secrets
        .into_iter()
        .map(|mut initial| {
            for _ in 0..2000 {
                initial = next_secret(initial);
            }
            initial as u64
        })
        .sum();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_22").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut all_secrets = contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(|initial| {
            let mut vec = vec![initial];
            vec.reserve(2000);
            for _ in 0..2000 {
                vec.push(next_secret(*vec.last().unwrap()));
            }
            vec
        })
        .collect_vec();

    let all_prizes_and_diffs = all_secrets
        .iter()
        .map(|secrets| {
            secrets
                .iter()
                .map(|secret| secret % 10)
                .tuple_windows()
                .map(|(last, current)| (current, current as i32 - last as i32))
                .collect_vec()
        })
        .collect_vec();

    let mut combinations: HashMap<(i32, i32, i32, i32), u32> = HashMap::new();

    for prizes_and_diffs in all_prizes_and_diffs.iter() {
        let mut current_combinations: HashMap<(i32, i32, i32, i32), u32> = HashMap::new();

        for ((_, diff1), (_, diff2), (_, diff3), (prize, diff4)) in
            prizes_and_diffs.iter().tuple_windows()
        {
            if !current_combinations.contains_key(&(*diff1, *diff2, *diff3, *diff4)) {
                current_combinations.insert((*diff1, *diff2, *diff3, *diff4), *prize);
            }
        }

        for (k, v) in current_combinations.drain() {
            combinations.insert(k, combinations.get(&k).unwrap_or(&&0) + v);
        }
    }

    let best = combinations
        .iter()
        .max_by_key(|(_, prize)| **prize)
        .unwrap();

    println!("{}", best.1);
}

fn next_secret(last: u32) -> u32 {
    let res = last;
    let res = res ^ (res << 6) % (1 << 24);
    let res = res ^ (res >> 5) % (1 << 24);
    let res = res ^ (res << 11) % (1 << 24);
    res
}
