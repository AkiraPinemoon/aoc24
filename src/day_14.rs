use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Read},
    ops::{Mul, Sub},
    path::Path,
};

use image::{ImageBuffer, Luma};
use itertools::Itertools;
use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_14").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots = re.captures_iter(&contents).filter_map(|cap| {
        let x = cap.get(1)?;
        let y = cap.get(2)?;
        let v_x = cap.get(3)?;
        let v_y = cap.get(4)?;

        Some((
            x.as_str().parse::<i64>().unwrap(),
            y.as_str().parse::<i64>().unwrap(),
            v_x.as_str().parse::<i64>().unwrap(),
            v_y.as_str().parse::<i64>().unwrap(),
        ))
    });

    let height = 103;
    let mid_h = (height - 1) / 2;
    let width = 101;
    let mid_w = (width - 1) / 2;
    let t = 100;

    let moved_robots = robots
        .map(|(x, y, v_x, v_y)| {
            (
                (x + v_x * t).rem_euclid(width),
                (y + v_y * t).rem_euclid(height),
            )
        })
        .collect_vec();

    let q1 = moved_robots
        .iter()
        .filter(|&&(x, y)| x < mid_w && y < mid_h)
        .count();

    let q2 = moved_robots
        .iter()
        .filter(|&&(x, y)| x > mid_w && y < mid_h)
        .count();

    let q3 = moved_robots
        .iter()
        .filter(|&&(x, y)| x < mid_w && y > mid_h)
        .count();

    let q4 = moved_robots
        .iter()
        .filter(|&&(x, y)| x > mid_w && y > mid_h)
        .count();

    let res = q1 * q2 * q3 * q4;

    println!("{} * {} * {} * {} = {}", q1, q2, q3, q4, res);
}

pub fn part_two() {
    let mut file = File::open("input_14").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots = re
        .captures_iter(&contents)
        .filter_map(|cap| {
            let x = cap.get(1)?;
            let y = cap.get(2)?;
            let v_x = cap.get(3)?;
            let v_y = cap.get(4)?;

            Some((
                x.as_str().parse::<i64>().unwrap(),
                y.as_str().parse::<i64>().unwrap(),
                v_x.as_str().parse::<i64>().unwrap(),
                v_y.as_str().parse::<i64>().unwrap(),
            ))
        })
        .collect_vec();

    let height = 103;
    let width = 101;

    let mut i = 0;
    loop {
        for entry in fs::read_dir("./14").expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let file_path = entry.path();
            if file_path.is_file() {
                fs::remove_file(file_path).expect("Failed to delete file");
            }
        }

        for _ in 0..100 {
            i += 1;
            robots = robots
                .into_iter()
                .map(|(x, y, v_x, v_y)| {
                    (
                        (x + v_x).rem_euclid(width),
                        (y + v_y).rem_euclid(height),
                        v_x,
                        v_y,
                    )
                })
                .collect_vec();

            println!("{}", i);
            save_img(width, height, &robots, format!("./14/{}.png", i));
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}

fn save_img(width: i64, height: i64, positions: &Vec<(i64, i64, i64, i64)>, path: String) {
    let mut img = ImageBuffer::from_pixel(width as u32, height as u32, Luma::<u8>([0]));

    for &(x, y, _, _) in positions.iter() {
        if x >= 0 && x < width && y >= 0 && y < height {
            img.put_pixel(x as u32, y as u32, Luma([255]));
        }
    }

    img.save(path).expect("Failed to save the image");
}
