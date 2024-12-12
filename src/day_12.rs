use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn part_one() {
    let mut file = File::open("input_12").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut regions: Vec<Region> = Vec::new();

    let mut map: HashMap<(usize, usize), char> = contents
        .lines()
        .enumerate()
        .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, ch)| ((x, y), ch)))
        .collect();

    while let Some(next_pos) = map.keys().next().cloned() {
        let next_plant = map.remove(&next_pos).unwrap();
        let mut region = Region {
            plant: next_plant,
            positions: HashSet::new(),
        };
        region.positions.insert(next_pos);

        region.propagate(next_pos.0, next_pos.1, &mut map);

        regions.push(region);
    }

    for region in regions.iter() {
        println!(
            "{} {} {}",
            region.plant,
            region.get_area(),
            region.get_border()
        );
    }

    let sum: usize = regions
        .iter()
        .map(|region| region.get_area() * region.get_border())
        .sum();

    println!("total: {}", sum)
}

pub fn part_two() {
    let mut file = File::open("input_12").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut regions: Vec<Region> = Vec::new();

    let mut map: HashMap<(usize, usize), char> = contents
        .lines()
        .enumerate()
        .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, ch)| ((x, y), ch)))
        .collect();

    while let Some(next_pos) = map.keys().next().cloned() {
        let next_plant = map.remove(&next_pos).unwrap();
        let mut region = Region {
            plant: next_plant,
            positions: HashSet::new(),
        };
        region.positions.insert(next_pos);

        region.propagate(next_pos.0, next_pos.1, &mut map);

        regions.push(region);
    }

    for region in regions.iter() {
        println!(
            "{} {} {}",
            region.plant,
            region.get_area(),
            region.get_sides()
        );
    }

    let sum: usize = regions
        .iter()
        .map(|region| region.get_area() * region.get_sides())
        .sum();

    println!("total: {}", sum)
}

#[derive(Debug)]
struct Region {
    pub plant: char,
    pub positions: HashSet<(usize, usize)>,
}

impl Region {
    pub fn is_adjacent(&self, x: usize, y: usize) -> bool {
        self.positions
            .iter()
            .any(|&(px, py)| (px == x && py.abs_diff(y) <= 1) || (py == y && px.abs_diff(x) <= 1))
    }

    pub fn get_area(&self) -> usize {
        self.positions.len()
    }

    pub fn propagate(&mut self, x: usize, y: usize, map: &mut HashMap<(usize, usize), char>) {
        if x > 0 {
            if let Some(check) = map.get(&(x - 1, y)) {
                if *check == self.plant {
                    self.positions.insert((x - 1, y));
                    map.remove(&(x - 1, y));
                    self.propagate(x - 1, y, map);
                }
            }
        }

        if let Some(check) = map.get(&(x + 1, y)) {
            if *check == self.plant {
                self.positions.insert((x + 1, y));
                map.remove(&(x + 1, y));
                self.propagate(x + 1, y, map);
            }
        }

        if y > 0 {
            if let Some(check) = map.get(&(x, y - 1)) {
                if *check == self.plant {
                    self.positions.insert((x, y - 1));
                    map.remove(&(x, y - 1));
                    self.propagate(x, y - 1, map);
                }
            }
        }

        if let Some(check) = map.get(&(x, y + 1)) {
            if *check == self.plant {
                self.positions.insert((x, y + 1));
                map.remove(&(x, y + 1));
                self.propagate(x, y + 1, map);
            }
        }
    }

    fn get_border(&self) -> usize {
        self.positions
            .iter()
            .cloned()
            .map(|pos| {
                let mut border = 0;

                if pos.0 == 0 || pos.0 > 0 && !self.positions.contains(&(pos.0 - 1, pos.1)) {
                    border += 1;
                }
                if !self.positions.contains(&(pos.0 + 1, pos.1)) {
                    border += 1;
                }

                if pos.1 == 0 || pos.1 > 0 && !self.positions.contains(&(pos.0, pos.1 - 1)) {
                    border += 1;
                }
                if !self.positions.contains(&(pos.0, pos.1 + 1)) {
                    border += 1;
                }

                border
            })
            .sum()
    }

    fn get_sides(&self) -> usize {
        let min_x = self.positions.iter().map(|pos| pos.0).min().unwrap();
        let max_x = self.positions.iter().map(|pos| pos.0).max().unwrap();
        let min_y = self.positions.iter().map(|pos| pos.1).min().unwrap();
        let max_y = self.positions.iter().map(|pos| pos.1).max().unwrap();

        let mut sides = 0;

        for x in min_x..=max_x + 1 {
            let mut on_border = false;
            for y in min_y..=max_y {
                if self.positions.contains(&(x, y))
                    && (x == 0 || !self.positions.contains(&(x - 1, y)))
                {
                    on_border = true;
                } else {
                    if on_border {
                        on_border = false;
                        sides += 1;
                    }
                }
            }
            if on_border {
                sides += 1;
            }

            on_border = false;
            for y in min_y..=max_y {
                if self.positions.contains(&(x, y)) && !self.positions.contains(&(x + 1, y)) {
                    on_border = true;
                } else {
                    if on_border {
                        on_border = false;
                        sides += 1;
                    }
                }
            }
            if on_border {
                sides += 1;
            }
        }

        for y in min_y..=max_y + 1 {
            let mut on_border = false;
            for x in min_x..=max_x {
                if self.positions.contains(&(x, y))
                    && (y == 0 || !self.positions.contains(&(x, y - 1)))
                {
                    on_border = true;
                } else {
                    if on_border {
                        on_border = false;
                        sides += 1;
                    }
                }
            }
            if on_border {
                sides += 1;
            }

            on_border = false;
            for x in min_x..=max_x {
                if self.positions.contains(&(x, y)) && !self.positions.contains(&(x, y + 1)) {
                    on_border = true;
                } else {
                    if on_border {
                        on_border = false;
                        sides += 1;
                    }
                }
            }
            if on_border {
                sides += 1;
            }
        }

        sides
    }
}
