use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Read,
};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_15").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map = contents
        .lines()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => Some(Tile::Wall),
                    'O' => Some(Tile::Box),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let instructions = contents
        .lines()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '^' => Dir::Up,
                'v' => Dir::Down,
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => panic!(),
            })
        })
        .collect_vec();

    let (mut y, line) = contents
        .lines()
        .find_position(|line| line.contains('@'))
        .unwrap();

    let mut x = line.chars().position(|ch| ch == '@').unwrap();

    print_map(&map, x, y);

    for instruction in instructions.into_iter() {
        // println!("moving {}", instruction);

        let (next_x, next_y) = match instruction {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };
        if push(next_x, next_y, instruction, &mut map) {
            x = next_x;
            y = next_y;
        }

        // print_map(&map, x, y);

        // let mut input = String::new();
        // std::io::stdin()
        // .read_line(&mut input)
        // .expect("Failed to read line");
    }

    print_map(&map, x, y);

    let sum = map
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(x, cell)| (x, y, cell))
        })
        .filter(|(x, y, cell)| matches!(cell, Some(Tile::Box)))
        .map(|(x, y, _)| x + 100 * y)
        .sum::<usize>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_15").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map = contents
        .lines()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .flat_map(|ch| match ch {
                    '#' => [Some(TileExt::Wall), Some(TileExt::Wall)],
                    'O' => [Some(TileExt::BoxL), Some(TileExt::BoxR)],
                    _ => [None, None],
                })
                .collect_vec()
        })
        .collect_vec();

    let instructions = contents
        .lines()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '^' => Dir::Up,
                'v' => Dir::Down,
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => panic!(),
            })
        })
        .collect_vec();

    let (mut y, line) = contents
        .lines()
        .find_position(|line| line.contains('@'))
        .unwrap();

    let mut x = line.chars().position(|ch| ch == '@').unwrap() * 2;

    print_map_ext(&map, x, y);

    for instruction in instructions.into_iter() {
        // println!("moving {}", instruction);

        let (next_x, next_y) = match instruction {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };
        if push_ext(next_x, next_y, instruction, &mut map) {
            x = next_x;
            y = next_y;
        }

        // print_map_ext(&map, x, y);

        // let mut input = String::new();
        // std::io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read line");
    }

    print_map_ext(&map, x, y);

    let sum = map
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(x, cell)| (x, y, cell))
        })
        .filter(|(x, y, cell)| matches!(cell, Some(TileExt::BoxL)))
        .map(|(x, y, _)| x + 100 * y)
        .sum::<usize>();

    println!("{}", sum);
}

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Box,
}

#[derive(Clone, Copy)]
enum TileExt {
    Wall,
    BoxL,
    BoxR,
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Dir::Up => "^",
            Dir::Down => "v",
            Dir::Left => "<",
            Dir::Right => ">",
        })
    }
}

fn print_map(map: &Vec<Vec<Option<Tile>>>, robot_x: usize, robot_y: usize) {
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if x == robot_x && y == robot_y {
                print!("@");
            } else {
                match cell {
                    Some(Tile::Wall) => print!("#"),
                    Some(Tile::Box) => print!("O"),
                    None => print!("."),
                }
            }
        }
        println!("");
    }
}

fn print_map_ext(map: &Vec<Vec<Option<TileExt>>>, robot_x: usize, robot_y: usize) {
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if x == robot_x && y == robot_y {
                print!("@");
            } else {
                match cell {
                    Some(TileExt::Wall) => print!("#"),
                    Some(TileExt::BoxL) => print!("["),
                    Some(TileExt::BoxR) => print!("]"),
                    None => print!("."),
                }
            }
        }
        println!("");
    }
}

fn push(x: usize, y: usize, dir: Dir, map: &mut Vec<Vec<Option<Tile>>>) -> bool {
    let size_x = map.first().unwrap().len();
    let size_y = map.len();

    if x >= size_x || y >= size_y {
        return false;
    }

    match map[y][x] {
        Some(tile) => match tile {
            Tile::Wall => false,
            Tile::Box => {
                let (next_x, next_y) = match dir {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                };
                if push(next_x, next_y, dir, map) {
                    map[next_y][next_x] = map[y][x];
                    map[y][x] = None;
                    true
                } else {
                    false
                }
            }
        },
        None => true,
    }
}

fn push_ext(x: usize, y: usize, dir: Dir, map: &mut Vec<Vec<Option<TileExt>>>) -> bool {
    let size_x = map.first().unwrap().len();
    let size_y = map.len();

    if x >= size_x || y >= size_y {
        return false;
    }

    match map[y][x] {
        Some(tile) => match tile {
            TileExt::Wall => false,
            TileExt::BoxL => {
                let (next_x, next_y) = match dir {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                };
                if matches!(dir, Dir::Up | Dir::Down) {
                    if test_push_ext(next_x, next_y, dir, map) && test_push_ext(next_x + 1, next_y, dir, map)
                    {
                        push_ext(next_x, next_y, dir, map);
                        push_ext(next_x + 1, next_y, dir, map);
                        map[next_y][next_x] = map[y][x];
                        map[y][x] = None;
                        map[next_y][next_x + 1] = map[y][x + 1];
                        map[y][x + 1] = None;
                        true
                    } else {
                        false
                    }
                } else {
                    if push_ext(next_x, next_y, dir, map) {
                        map[next_y][next_x] = map[y][x];
                        map[y][x] = None;
                        true
                    } else {
                        false
                    }
                }
            }
            TileExt::BoxR => {
                let (next_x, next_y) = match dir {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                };
                if matches!(dir, Dir::Up | Dir::Down) {
                    if test_push_ext(next_x, next_y, dir, map) && test_push_ext(next_x - 1, next_y, dir, map)
                    {
                        push_ext(next_x, next_y, dir, map);
                        push_ext(next_x - 1, next_y, dir, map);
                        map[next_y][next_x] = map[y][x];
                        map[y][x] = None;
                        map[next_y][next_x - 1] = map[y][x - 1];
                        map[y][x - 1] = None;
                        true
                    } else {
                        false
                    }
                } else {
                    if push_ext(next_x, next_y, dir, map) {
                        map[next_y][next_x] = map[y][x];
                        map[y][x] = None;
                        true
                    } else {
                        false
                    }
                }
            }
        },
        None => true,
    }
}

fn test_push_ext(x: usize, y: usize, dir: Dir, map: &Vec<Vec<Option<TileExt>>>) -> bool {
    let size_x = map.first().unwrap().len();
    let size_y = map.len();

    if x >= size_x || y >= size_y {
        return false;
    }

    match map[y][x] {
        Some(tile) => match tile {
            TileExt::Wall => false,
            TileExt::BoxL => {
                let (next_x, next_y) = match dir {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                };
                if matches!(dir, Dir::Up | Dir::Down) {
                    if test_push_ext(next_x, next_y, dir, map) && test_push_ext(next_x + 1, next_y, dir, map)
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    if test_push_ext(next_x, next_y, dir, map) {
                        true
                    } else {
                        false
                    }
                }
            }
            TileExt::BoxR => {
                let (next_x, next_y) = match dir {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                };
                if matches!(dir, Dir::Up | Dir::Down) {
                    if test_push_ext(next_x, next_y, dir, map) && test_push_ext(next_x - 1, next_y, dir, map)
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    if test_push_ext(next_x, next_y, dir, map) {
                        true
                    } else {
                        false
                    }
                }
            }
        },
        None => true,
    }
}
