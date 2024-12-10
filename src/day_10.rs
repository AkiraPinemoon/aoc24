use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    fs::File,
    io::Read,
};

pub fn part_one() {
    let mut file = File::open("input_10").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let map: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut heads = map
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(y, cell)| if *cell == 0 { Some(((x, y), 0)) } else { None })
        })
        .collect::<HashMap<_, _>>();

    for (head, score) in heads.iter_mut() {
        *score = hike(*head, &map).len();
        println!("found {} trails for ({}, {})", score, head.0, head.1);
    }

    let sum = heads.values().sum::<usize>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_10").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let map: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut heads = map
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(y, cell)| if *cell == 0 { Some(((x, y), 0)) } else { None })
        })
        .collect::<HashMap<_, _>>();

    for (head, score) in heads.iter_mut() {
        *score = hike_rating(*head, &map);
        println!("found {} trails for ({}, {})", score, head.0, head.1);
    }

    let sum = heads.values().sum::<usize>();

    println!("{}", sum);
}

fn hike(start: (usize, usize), map: &Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let current_height = map[start.0][start.1];
    if current_height == 9 {
        return HashSet::from([start]);
    }

    let next_height = current_height + 1;
    let mut res = HashSet::new();
    if start.0 > 0 && map[start.0 - 1][start.1] == next_height {
        for pos in hike((start.0 - 1, start.1), map).into_iter() {
            res.insert(pos);
        }
    }

    if start.1 > 0 && map[start.0][start.1 - 1] == next_height {
        for pos in hike((start.0, start.1 - 1), map).into_iter() {
            res.insert(pos);
        }
    }

    if start.0 + 1 < map.len() && map[start.0 + 1][start.1] == next_height {
        for pos in hike((start.0 + 1, start.1), map).into_iter() {
            res.insert(pos);
        }
    }

    if start.1 + 1 < map[0].len() && map[start.0][start.1 + 1] == next_height {
        for pos in hike((start.0, start.1 + 1), map).into_iter() {
            res.insert(pos);
        }
    }

    res
}

fn hike_rating(start: (usize, usize), map: &Vec<Vec<u32>>) -> usize {
    let current_height = map[start.0][start.1];
    if current_height == 9 {
        return 1;
    }

    let next_height = current_height + 1;
    let mut res = 0;
    if start.0 > 0 && map[start.0 - 1][start.1] == next_height {
        res += hike_rating((start.0 - 1, start.1), map);
    }

    if start.1 > 0 && map[start.0][start.1 - 1] == next_height {
        res += hike_rating((start.0, start.1 - 1), map);
    }

    if start.0 + 1 < map.len() && map[start.0 + 1][start.1] == next_height {
        res += hike_rating((start.0 + 1, start.1), map);
    }

    if start.1 + 1 < map[0].len() && map[start.0][start.1 + 1] == next_height {
        res += hike_rating((start.0, start.1 + 1), map);
    }

    res
}
