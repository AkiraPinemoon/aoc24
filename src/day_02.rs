use std::{fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_02").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|inp| {
            let mut report = inp.clone();
            if report.len() < 2 {
                panic!("report too short")
            }
            let mut current = report.pop().unwrap();
            let dir = *report.last().unwrap() > current;

            while let Some(next) = report.pop() {
                if (next > current) != dir {
                    return false;
                }
                if next.abs_diff(current) < 1 || next.abs_diff(current) > 3 {
                    return false;
                }
                current = next;
            }
            true
        })
        .count();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_02").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|inp| {
            for idx in 0..inp.len() {
                let mut test = inp.clone();
                test.remove(idx);
                if safety_test(test) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("{}", sum);
}

fn safety_test(mut report: Vec<i32>) -> bool {
    if report.len() < 2 {
        panic!("report too short")
    }
    let mut current = report.pop().unwrap();
    let dir = *report.last().unwrap() > current;

    while let Some(next) = report.pop() {
        if (next > current) != dir {
            return false;
        }
        if next.abs_diff(current) < 1 || next.abs_diff(current) > 3 {
            return false;
        }
        current = next;
    }
    true
}
