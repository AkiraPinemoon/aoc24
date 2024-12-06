use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

pub fn part_one() {
    let mut file = File::open("input_06").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let (mut x, mut y, dir_char) = contents
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, char)| (x as isize, y as isize, char))
        })
        .find(|(_, _, ch)| matches!(ch, '^' | '>' | 'v' | '<'))
        .unwrap();

    let mut dir: Dir = dir_char.try_into().unwrap();

    let map: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|ch| matches!(ch, '#')).collect())
        .collect();

    let (size_x, size_y) = (map.len() as isize, map.first().unwrap().len() as isize);

    let mut visited = HashSet::new();
    visited.insert((x, y));

    loop {
        let (next_x, next_y) = match dir {
            Dir::Up => (x - 1, y),
            Dir::Right => (x, y + 1),
            Dir::Down => (x + 1, y),
            Dir::Left => (x, y - 1),
        };

        if next_x < 0 || next_x >= size_x || next_y < 0 || next_y >= size_y {
            break;
        }

        if map[next_x as usize][next_y as usize] {
            dir.rotate();
        } else {
            (x, y) = (next_x, next_y);
            visited.insert((x, y));
        }
    }

    println!("{}", visited.len());
}

pub fn part_two() {
    let mut file = File::open("input_06").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let (mut x, mut y, dir_char) = contents
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, char)| (x as isize, y as isize, char))
        })
        .find(|(_, _, ch)| matches!(ch, '^' | '>' | 'v' | '<'))
        .unwrap();

    let mut dir: Dir = dir_char.try_into().unwrap();

    let map: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|ch| matches!(ch, '#')).collect())
        .collect();

    let (size_x, size_y) = (map.len() as isize, map.first().unwrap().len() as isize);

    let mut loops = HashSet::new();

    let mut visited = HashSet::new();
    visited.insert((x, y));

    loop {
        let (next_x, next_y) = match dir {
            Dir::Up => (x - 1, y),
            Dir::Right => (x, y + 1),
            Dir::Down => (x + 1, y),
            Dir::Left => (x, y - 1),
        };

        if next_x < 0 || next_x >= size_x || next_y < 0 || next_y >= size_y {
            break;
        }

        if map[next_x as usize][next_y as usize] {
            dir.rotate();
        } else {
            let mut changed = map.clone();
            changed[next_x as usize][next_y as usize] = true;
            if !visited.contains(&(next_x, next_y)) {
                if is_cyclic(x, y, dir, changed) {
                    loops.insert((next_x, next_y));
                }
            }

            (x, y) = (next_x, next_y);
            visited.insert((x, y));
        }
    }

    println!("{}", loops.len());
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

impl Dir {
    pub fn rotate(&mut self) {
        match self {
            Dir::Up => *self = Dir::Right,
            Dir::Right => *self = Dir::Down,
            Dir::Down => *self = Dir::Left,
            Dir::Left => *self = Dir::Up,
        }
    }
}

fn is_cyclic(mut x: isize, mut y: isize, mut dir: Dir, map: Vec<Vec<bool>>) -> bool {
    let (size_x, size_y) = (map.len() as isize, map.first().unwrap().len() as isize);

    let mut visited = HashSet::new();
    visited.insert((x, y, dir));

    loop {
        let (next_x, next_y) = match dir {
            Dir::Up => (x - 1, y),
            Dir::Right => (x, y + 1),
            Dir::Down => (x + 1, y),
            Dir::Left => (x, y - 1),
        };

        if next_x < 0 || next_x >= size_x || next_y < 0 || next_y >= size_y {
            return false;
        }

        if map[next_x as usize][next_y as usize] {
            dir.rotate();
            if visited.contains(&(x, y, dir)) {
                return true;
            }
            visited.insert((x, y, dir));
        } else {
            (x, y) = (next_x, next_y);
            if visited.contains(&(x, y, dir)) {
                return true;
            }
            visited.insert((x, y, dir));
        }
    }
}
