use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Read,
    usize,
};

use image::{ImageBuffer, Luma, Rgb};
use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_16").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let buffer = contents
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#').collect_vec())
        .collect_vec();

    let (start_y, line) = contents
        .lines()
        .find_position(|line| line.contains('S'))
        .unwrap();
    let start_x = line.chars().position(|cell| cell == 'S').unwrap();

    let (end_y, line) = contents
        .lines()
        .find_position(|line| line.contains('E'))
        .unwrap();
    let end_x = line.chars().position(|cell| cell == 'E').unwrap();

    let width = buffer.first().unwrap().len();
    let height = buffer.len();

    let mut map = Map {
        buffer,
        height,
        width,
        start: (start_x, start_y),
        end: (end_x, end_y),
    };

    // map.end.0 = map.start.0 + 2;
    // map.end.1 = map.start.1 - 2;

    save_map(&map, "./16.png".to_owned());

    let path = find_optimal_path(&map, &mut Vec::new(), u32::MAX).unwrap();
    println!("{:#?}", path);
}

pub fn part_two() {}

struct Map {
    buffer: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

fn save_map(map: &Map, path: String) {
    let mut img =
        ImageBuffer::from_pixel(map.width as u32, map.height as u32, Rgb::<u8>([0, 0, 0]));

    for (x, line) in map.buffer.iter().cloned().enumerate() {
        for (y, cell) in line.iter().enumerate() {
            if *cell {
                img.put_pixel(y as u32, x as u32, Rgb([255, 255, 255]));
            }
        }
    }

    img.put_pixel(map.start.0 as u32, map.start.1 as u32, Rgb([0, 255, 0]));
    img.put_pixel(map.end.0 as u32, map.end.1 as u32, Rgb([255, 0, 0]));

    img.save(path).expect("Failed to save the image");
}

fn find_optimal_path(
    map: &Map,
    steps: &mut Vec<(usize, usize, Dir, u32)>,
    mut shortest: u32,
) -> Option<Vec<(usize, usize, Dir, u32)>> {
    let (x, y, dir, score) =
        steps
            .last()
            .cloned()
            .unwrap_or((map.start.0, map.start.1, Dir::Right, 0));

    if (x, y) == map.end {
        return Some(steps.clone());
    }

    if score > shortest {
        return None;
    }

    // println!("searching {:#?}", steps);

    let mut shortest_path: Option<Vec<(usize, usize, Dir, u32)>> = None;

    // up
    if y != 0 && !map.buffer[y - 1][x] {
        let next_score = if dir == Dir::Up {
            score + 1
        } else {
            score + 1001
        };

        if !steps
            .iter()
            .any(|&(s_x, s_y, _s_dir, _s_scoree)| s_x == x && s_y == y - 1 && score <= next_score)
        {
            steps.push((x, y - 1, Dir::Up, next_score));
            if let Some(path) = find_optimal_path(map, steps, shortest) {
                if let Some(current) = &shortest_path {
                    if current.last().unwrap().3 > path.last().unwrap().3 {
                        shortest = path.last().unwrap().3;
                        shortest_path = Some(path);
                    }
                } else {
                    shortest = path.last().unwrap().3;
                    shortest_path = Some(path);
                }
            }
            steps.pop();
        }
    }

    // down
    if y < map.height && !map.buffer[y + 1][x] {
        let next_score = if dir == Dir::Down {
            score + 1
        } else {
            score + 1001
        };

        if !steps
            .iter()
            .any(|&(s_x, s_y, _s_dir, _s_scoree)| s_x == x && s_y == y + 1 && score <= next_score)
        {
            steps.push((x, y + 1, Dir::Down, next_score));
            if let Some(path) = find_optimal_path(map, steps, shortest) {
                if let Some(current) = &shortest_path {
                    if current.last().unwrap().3 > path.last().unwrap().3 {
                        shortest = path.last().unwrap().3;
                        shortest_path = Some(path);
                    }
                } else {
                    shortest = path.last().unwrap().3;
                    shortest_path = Some(path);
                }
            }
            steps.pop();
        }
    }

    // left
    if x != 0 && !map.buffer[y][x - 1] {
        let next_score = if dir == Dir::Left {
            score + 1
        } else {
            score + 1001
        };

        if !steps
            .iter()
            .any(|&(s_x, s_y, _s_dir, _s_scoree)| s_x == x - 1 && s_y == y && score <= next_score)
        {
            steps.push((x - 1, y, Dir::Left, next_score));
            if let Some(path) = find_optimal_path(map, steps, shortest) {
                if let Some(current) = &shortest_path {
                    if current.last().unwrap().3 > path.last().unwrap().3 {
                        shortest = path.last().unwrap().3;
                        shortest_path = Some(path);
                    }
                } else {
                    shortest = path.last().unwrap().3;
                    shortest_path = Some(path);
                }
            }
            steps.pop();
        }
    }

    // right
    if x < map.width && !map.buffer[y][x + 1] {
        let next_score = if dir == Dir::Right {
            score + 1
        } else {
            score + 1001
        };

        if !steps
            .iter()
            .any(|&(s_x, s_y, _s_dir, _s_scoree)| s_x == x + 1 && s_y == y && score <= next_score)
        {
            steps.push((x + 1, y, Dir::Right, next_score));
            if let Some(path) = find_optimal_path(map, steps, shortest) {
                if let Some(current) = &shortest_path {
                    if current.last().unwrap().3 > path.last().unwrap().3 {
                        shortest = path.last().unwrap().3;
                        shortest_path = Some(path);
                    }
                } else {
                    shortest = path.last().unwrap().3;
                    shortest_path = Some(path);
                }
            }
            steps.pop();
        }
    }

    if let Some(path) = &shortest_path {
        println!("{:#?}", path);
    }

    shortest_path
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
