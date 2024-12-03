use std::{collections::HashMap, fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_01").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in contents.lines() {
        let mut values = line
            .split_whitespace()
            .map(|value| value.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if values.len() != 2 {
            panic!("couldn't parse pair from: {}", line);
        }

        right.push(values.pop().unwrap());
        left.push(values.pop().unwrap());
    }

    left.sort();
    right.sort();

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| difference(l, r))
        .sum();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_01").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in contents.lines() {
        let mut values = line
            .split_whitespace()
            .map(|value| value.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if values.len() != 2 {
            panic!("couldn't parse pair from: {}", line);
        }

        right.push(values.pop().unwrap());
        left.push(values.pop().unwrap());
    }

    let mut r_count = HashMap::new();

    for value in right.into_iter() {
        r_count.insert(value, (r_count.get(&value).unwrap_or(&0) + 1));
    }

    let sum: i32 = left.iter().map(|l| l * r_count.get(l).unwrap_or(&0)).sum();

    println!("{}", sum);
}

fn difference(l: i32, r: i32) -> i32 {
    if l > r {
        l - r
    } else {
        r - l
    }
}
