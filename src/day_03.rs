use std::{fs::File, io::Read};

use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_03").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum = re
        .captures_iter(&contents)
        .map(|capture| capture.extract())
        .map(|(_, [x, y])| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .map(|(x, y)| x * y)
        .sum::<u32>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_03").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let mut condition = true;

    let sum = re
        .captures_iter(&contents)
        .filter(|cap| {
            if cap.get(4).is_some() {
                condition = true
            } else if cap.get(5).is_some() {
                condition = false
            } else {
                return condition;
            }
            false
        })
        .map(|cap| (cap.get(2).unwrap().as_str().parse::<u32>().unwrap(), cap.get(3).unwrap().as_str().parse::<u32>().unwrap()))
        .map(|(x, y)| x * y)
        .sum::<u32>();

    println!("{}", sum);
}
