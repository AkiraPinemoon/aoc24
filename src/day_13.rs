use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    ops::{Mul, Sub},
};

use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_13").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let games = re.captures_iter(&contents).filter_map(|cap| {
        let ax = cap.get(1)?.as_str().parse().ok()?;
        let ay = cap.get(2)?.as_str().parse().ok()?;

        let bx = cap.get(3)?.as_str().parse().ok()?;
        let by = cap.get(4)?.as_str().parse().ok()?;

        let px = cap.get(5)?.as_str().parse().ok()?;
        let py = cap.get(6)?.as_str().parse().ok()?;

        Some(Game {
            a: (ax, ay).into(),
            b: (bx, by).into(),
            prize: (px, py).into(),
        })
    });

    let sum = games
        .filter_map(|game| {
            let mut min = None;

            let mut a_count = 0;

            while a_count * game.a.x <= game.prize.x && a_count * game.a.y <= game.prize.y {
                let remainder = game.prize - game.a * a_count;

                if remainder.x % game.b.x == 0 && remainder.y % game.b.y == 0 {
                    let b_count = remainder.x / game.b.x;
                    if game.b.y * b_count == remainder.y {
                        match min {
                            None => {
                                min = Some(a_count * 3 + b_count);
                            }
                            Some(val) => {
                                let found = a_count * 3 + b_count;
                                if found < val {
                                    min = Some(found);
                                }
                            }
                        }
                    }
                }

                a_count += 1;
            }

            min
        })
        .sum::<i64>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_13").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let games = re.captures_iter(&contents).filter_map(|cap| {
        let ax = cap.get(1)?.as_str().parse().ok()?;
        let ay = cap.get(2)?.as_str().parse().ok()?;

        let bx = cap.get(3)?.as_str().parse().ok()?;
        let by = cap.get(4)?.as_str().parse().ok()?;

        let mut px = cap.get(5)?.as_str().parse().ok()?;
        let mut py = cap.get(6)?.as_str().parse().ok()?;

        px += 10000000000000;
        py += 10000000000000;

        Some(Game {
            a: (ax, ay).into(),
            b: (bx, by).into(),
            prize: (px, py).into(),
        })
    });

    let sum = games
        .filter_map(|game| {
            println!(
                "searching for ({}, {}) using ({}, {}) and ({}, {})",
                game.prize.x, game.prize.y, game.a.x, game.a.y, game.b.x, game.b.y
            );

            solve_linear_combination(game.a, game.b, game.prize)
        })
        .map(|(a_count, b_count)| a_count * 3 + b_count)
        .sum::<i64>();

    println!("{}", sum);
}

struct Game {
    pub a: Vec2,
    pub b: Vec2,
    pub prize: Vec2,
}

#[derive(Clone, Copy)]
struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(i64, i64)> for Vec2 {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Into<(i64, i64)> for Vec2 {
    fn into(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

fn solve_linear_combination(a: Vec2, b: Vec2, c: Vec2) -> Option<(i64, i64)> {
    let det = a.x * b.y - a.y * b.x;

    if det == 0 {
        return None;
    }

    let det_x = c.x * b.y - c.y * b.x;
    let det_y = a.x * c.y - a.y * c.x;

    if det_x % det != 0 || det_y % det != 0 {
        return None;
    }

    let x = det_x / det;
    let y = det_y / det;

    Some((x, y))
}
