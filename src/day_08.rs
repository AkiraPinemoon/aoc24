use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    fs::File,
    io::Read,
};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_08").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut frequencies = HashMap::new();
    let mut next_frequency = 0;

    let map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch == '.' {
                        return None;
                    } else {
                        match frequencies.get(&ch) {
                            Some(val) => return Some(*val),
                            None => {
                                let freq = next_frequency;
                                next_frequency += 1;
                                frequencies.insert(ch, freq);
                                return Some(freq);
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let size = (map.len(), map.first().unwrap().len());

    let antennas = map
        .into_iter()
        .enumerate()
        .flat_map(|(x, line)| line.into_iter().enumerate().map(move |(y, cell)| (x as isize, y as isize, cell)))
        .filter_map(|(x, y, cell)| match cell {
            Some(frequency) => Some((x, y, frequency)),
            None => None,
        })
        .collect::<Vec<_>>();

    let mut antinodes = HashSet::new();

    for frequency in 0..next_frequency {
        for (a, b) in antennas
            .iter()
            .filter(|(x, z, freq)| *freq == frequency)
            .tuple_combinations() {
                let a_to_b = (b.0 - a.0, b.1 - a.1);
                let antinode_a = (b.0 + a_to_b.0, b.1 + a_to_b.1);
                let antinode_b = (a.0 - a_to_b.0, a.1 - a_to_b.1);
                antinodes.insert(antinode_a);
                antinodes.insert(antinode_b);
            }
    }

    let antinodes_on_map = antinodes.into_iter().filter(|(x, y)| *x < size.0 as isize && *y < size.1 as isize && *x >= 0 && *y >= 0).count();

    println!("{}", antinodes_on_map);
}

pub fn part_two() {
    let mut file = File::open("input_08").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut frequencies = HashMap::new();
    let mut next_frequency = 0;

    let map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch == '.' {
                        return None;
                    } else {
                        match frequencies.get(&ch) {
                            Some(val) => return Some(*val),
                            None => {
                                let freq = next_frequency;
                                next_frequency += 1;
                                frequencies.insert(ch, freq);
                                return Some(freq);
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let size = (map.len(), map.first().unwrap().len());

    let antennas = map
        .into_iter()
        .enumerate()
        .flat_map(|(x, line)| line.into_iter().enumerate().map(move |(y, cell)| (x as isize, y as isize, cell)))
        .filter_map(|(x, y, cell)| match cell {
            Some(frequency) => Some((x, y, frequency)),
            None => None,
        })
        .collect::<Vec<_>>();

    let mut antinodes = HashSet::new();

    for frequency in 0..next_frequency {
        for (a, b) in antennas
            .iter()
            .filter(|(x, z, freq)| *freq == frequency)
            .tuple_combinations() {
                let a_to_b = (b.0 - a.0, b.1 - a.1);

                let mut last = (b.0, b.1);
                while last.0 < size.0 as isize && last.1 < size.1 as isize && last.0 >= 0 && last.1 >= 0 {
                    antinodes.insert(last);
                    last = (last.0 + a_to_b.0, last.1 + a_to_b.1);
                }

                last = (a.0, a.1);
                while last.0 < size.0 as isize && last.1 < size.1 as isize && last.0 >= 0 && last.1 >= 0 {
                    antinodes.insert(last);
                    last = (last.0 - a_to_b.0, last.1 - a_to_b.1);
                }
            }
    }

    println!("{}", antinodes.len());
}
