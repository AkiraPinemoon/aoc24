use std::{fs::File, io::Read};

use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_04").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let search: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let sum = search
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(y, ch)| **ch == 'X')
                .map(|(y, ch)| check_xmas(x, y, &search))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_04").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let search: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let sum = search
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(y, ch)| **ch == 'A')
                .filter(|(y, ch)| check_mas(x, *y, &search))
                .count()
        })
        .sum::<usize>();

    println!("{}", sum);
}

fn check_xmas(x: usize, y: usize, search: &Vec<Vec<char>>) -> u32 {
    if search.len() < 1 {
        panic!();
    }
    if search.first().unwrap().len() < 1 {
        panic!();
    }
    let size = (search.len(), search.first().unwrap().len());

    let mut res = 0;

    // pos x
    if x + 3 < size.0 {
        if is_xmas((search[x + 1][y], search[x + 2][y], search[x + 3][y])) {
            res += 1;
        }
    }
    // neg x
    if x >= 3 {
        if is_xmas((search[x - 1][y], search[x - 2][y], search[x - 3][y])) {
            res += 1;
        }
    }
    // pos y
    if y + 3 < size.1 {
        if is_xmas((search[x][y + 1], search[x][y + 2], search[x][y + 3])) {
            res += 1;
        }
    }
    // neg y
    if y >= 3 {
        if is_xmas((search[x][y - 1], search[x][y - 2], search[x][y - 3])) {
            res += 1;
        }
    }
    // diagonal pos pos
    if x + 3 < size.0 && y + 3 < size.1 {
        if is_xmas((
            search[x + 1][y + 1],
            search[x + 2][y + 2],
            search[x + 3][y + 3],
        )) {
            res += 1;
        }
    }
    // diagonal pos neg
    if x + 3 < size.0 && y >= 3 {
        if is_xmas((
            search[x + 1][y - 1],
            search[x + 2][y - 2],
            search[x + 3][y - 3],
        )) {
            res += 1;
        }
    }
    // diagonal neg pos
    if x >= 3 && y + 3 < size.1 {
        if is_xmas((
            search[x - 1][y + 1],
            search[x - 2][y + 2],
            search[x - 3][y + 3],
        )) {
            res += 1;
        }
    }
    // diagonal neg neg
    if x >= 3 && y >= 3 {
        if is_xmas((
            search[x - 1][y - 1],
            search[x - 2][y - 2],
            search[x - 3][y - 3],
        )) {
            res += 1;
        }
    }

    res
}

fn is_xmas(input: (char, char, char)) -> bool {
    matches!(input, ('M', 'A', 'S'))
}

fn check_mas(x: usize, y: usize, search: &Vec<Vec<char>>) -> bool {
    if search.len() < 1 {
        panic!();
    }
    if search.first().unwrap().len() < 1 {
        panic!();
    }
    let size = (search.len(), search.first().unwrap().len());

    let mut res = 0;

    if x + 1 < size.0 && x >= 1 && y + 1 < size.1 && y >= 1 {
        // diagonal pos pos
        if is_mas((search[x - 1][y - 1], search[x + 1][y + 1])) {
            res += 1;
        }
        // diagonal neg neg
        if is_mas((search[x + 1][y + 1], search[x - 1][y - 1])) {
            res += 1;
        }
        // diagonal pos neg
        if is_mas((search[x - 1][y + 1], search[x + 1][y - 1])) {
            res += 1;
        }
        // diagonal neg pos
        if is_mas((search[x + 1][y - 1], search[x - 1][y + 1])) {
            res += 1;
        }
    }

    res >= 2
}

fn is_mas(input: (char, char)) -> bool {
    matches!(input, ('M', 'S'))
}
