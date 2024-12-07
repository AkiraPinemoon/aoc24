use std::{fmt::format, fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_07").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum = contents
        .lines()
        .map(|line| {
            let mut iter = line.split(": ");
            let res = iter.next().unwrap().parse().unwrap();
            let vals = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect::<Vec<_>>();
            (res, vals)
        })
        .filter(|(res, vals)| can_calc(*res, vals.clone()))
        .map(|(res, _)| res)
        .sum::<u64>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_07").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum = contents
        .lines()
        .map(|line| {
            let mut iter = line.split(": ");
            let res = iter.next().unwrap().parse().unwrap();
            let vals = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect::<Vec<_>>();
            (res, vals)
        })
        .filter(|(res, vals)| can_calc_2(*res, vals.clone()))
        .map(|(res, _)| res)
        .sum::<u64>();

    println!("{}", sum);
}

fn can_calc(res: u64, mut vals: Vec<u64>) -> bool {
    if vals.len() == 1 {
        return res == vals.pop().unwrap();
    }

    let next = vals.pop().unwrap();
    if res % next == 0 {
        if can_calc(res / next, vals.clone()) {
            return true;
        }
    }

    if res < next {
        return false;
    }
    return can_calc(res - next, vals.clone());
}

fn can_calc_2(res: u64, mut vals: Vec<u64>) -> bool {
    if vals.len() == 1 {
        return res == vals.pop().unwrap();
    }

    let next = vals.pop().unwrap();
    if res % next == 0 {
        if can_calc_2(res / next, vals.clone()) {
            return true;
        }
    }

    if res < next {
        return false;
    }
    if can_calc_2(res - next, vals.clone()) {
        return true;
    }

    let res_str = format!("{}", res);
    let next_str = format!("{}", next);

    if res_str.ends_with(&next_str) {
        if let Ok(new_res) = res_str.strip_suffix(&next_str).unwrap().parse() {
            return can_calc_2(new_res, vals);
        }
    }
    false
}
