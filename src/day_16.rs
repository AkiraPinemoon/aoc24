use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    fs::File,
    io::Read,
    usize,
};

use ggez::{
    conf::{WindowMode, WindowSetup},
    event,
    graphics::{self, Color, DrawParam, FillOptions, Mesh, MeshBuilder, Rect},
    mint::Point2,
    ContextBuilder,
};
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

    let map = Map {
        buffer,
        height,
        width,
        start: (start_x, start_y),
        end: (end_x, end_y),
    };

    let mut paths = Vec::new();
    for _ in 0..map.height {
        let mut l = Vec::new();
        for _ in 0..map.width {
            l.push(None);
        }
        paths.push(l);
    }

    paths[map.start.1][map.start.0] = Some((Dir::Right, 0));

    let stack = VecDeque::from([(map.start.0, map.start.1)]);

    let cb = ContextBuilder::new("day 16", "akira").window_mode(WindowMode::dimensions(
        WindowMode::default(),
        (map.width * 5) as f32,
        (map.height * 5) as f32,
    ));
    let (ctx, event_loop) = cb.build().unwrap();
    let state = MainState {
        map,
        paths,
        stack,
        solved: false,
    };
    event::run(ctx, event_loop, state);
}

pub fn part_two() {
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

    let map = Map {
        buffer,
        height,
        width,
        start: (start_x, start_y),
        end: (end_x, end_y),
    };

    let mut paths = Vec::new();
    for _ in 0..map.height {
        let mut l = Vec::new();
        for _ in 0..map.width {
            l.push(None);
        }
        paths.push(l);
    }

    paths[map.start.1][map.start.0] = Some((Dir::Right, 0));

    let stack = VecDeque::from([(map.start.0, map.start.1)]);

    let cb = ContextBuilder::new("day 16", "akira").window_mode(WindowMode::dimensions(
        WindowMode::default(),
        (map.width * 5) as f32,
        (map.height * 5) as f32,
    ));
    let (ctx, event_loop) = cb.build().unwrap();
    let state = MainState2 {
        map,
        paths,
        stack,
        solved: false,
        cells_on_path: HashSet::new(),
    };
    event::run(ctx, event_loop, state);
}

struct Map {
    buffer: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn propagate(
    map: &Map,
    x: usize,
    y: usize,
    paths: &mut Vec<Vec<Option<(Dir, usize)>>>,
    stack: &mut VecDeque<(usize, usize)>,
) {
    let (dir, score) = paths[y][x].unwrap();

    // Check UP (if the last move was NOT DOWN)
    if dir != Dir::Down && y > 0 && !map.buffer[y - 1][x] {
        let next_score = if dir == Dir::Up {
            score + 1
        } else {
            score + 1001
        };
        if paths[y - 1][x].is_none() || paths[y - 1][x].unwrap().1 > next_score {
            paths[y - 1][x] = Some((Dir::Up, next_score));
            stack.push_back((x, y - 1));
        }
    }

    // Check DOWN (if the last move was NOT UP)
    if dir != Dir::Up && y < map.height - 1 && !map.buffer[y + 1][x] {
        let next_score = if dir == Dir::Down {
            score + 1
        } else {
            score + 1001
        };
        if paths[y + 1][x].is_none() || paths[y + 1][x].unwrap().1 > next_score {
            paths[y + 1][x] = Some((Dir::Down, next_score));
            stack.push_back((x, y + 1));
        }
    }

    // Check LEFT (if the last move was NOT RIGHT)
    if dir != Dir::Right && x > 0 && !map.buffer[y][x - 1] {
        let next_score = if dir == Dir::Left {
            score + 1
        } else {
            score + 1001
        };
        if paths[y][x - 1].is_none() || paths[y][x - 1].unwrap().1 > next_score {
            paths[y][x - 1] = Some((Dir::Left, next_score));
            stack.push_back((x - 1, y));
        }
    }

    // Check RIGHT (if the last move was NOT LEFT)
    if dir != Dir::Left && x < map.width - 1 && !map.buffer[y][x + 1] {
        let next_score = if dir == Dir::Right {
            score + 1
        } else {
            score + 1001
        };
        if paths[y][x + 1].is_none() || paths[y][x + 1].unwrap().1 > next_score {
            paths[y][x + 1] = Some((Dir::Right, next_score));
            stack.push_back((x + 1, y));
        }
    }
}

struct MainState {
    map: Map,
    paths: Vec<Vec<Option<(Dir, usize)>>>,
    stack: VecDeque<(usize, usize)>,
    solved: bool,
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut max_steps = 1000;
        while let Some(next) = self.stack.pop_front() {
            max_steps -= 1;
            propagate(&self.map, next.0, next.1, &mut self.paths, &mut self.stack);

            if self.stack.is_empty() {
                self.solved = true;
                let score = self.paths[self.map.end.1][self.map.end.0].unwrap().1;
                println!("{}", score);
            }

            if max_steps < 0 {
                return Ok(());
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        let mut mb = MeshBuilder::new();

        for (y, line) in self.map.buffer.iter().cloned().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    mb.rectangle(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                        Color::from_rgb(255, 255, 255),
                    )
                    .unwrap();
                } else if let Some((dir, score)) = self.paths[y][x] {
                    let points = match dir {
                        Dir::Down => [
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 2.5,
                                y: (y * 5) as f32 + 5.0,
                            },
                        ],
                        Dir::Up => [
                            Point2 {
                                x: (x * 5) as f32 + 2.5,
                                y: (y * 5) as f32,
                            },
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32 + 5.0,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32 + 5.0,
                            },
                        ],
                        Dir::Left => [
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32 + 2.5,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32 + 5.0,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32,
                            },
                        ],
                        Dir::Right => [
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32 + 2.5,
                            },
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32 + 5.0,
                            },
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32,
                            },
                        ],
                    };

                    mb.polygon(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        &points,
                        Color::from_rgb(0, 0, 255),
                    )
                    .unwrap();
                }
            }
        }

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                (self.map.start.0 * 5) as f32,
                (self.map.start.1 * 5) as f32,
                5.0,
                5.0,
            ),
            Color::from_rgb(0, 255, 0),
        )
        .unwrap();

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                (self.map.end.0 * 5) as f32,
                (self.map.end.1 * 5) as f32,
                5.0,
                5.0,
            ),
            Color::from_rgb(255, 0, 0),
        )
        .unwrap();

        if self.solved {
            let (mut x, mut y) = self.map.end;

            while (x, y) != self.map.start {
                mb.rectangle(
                    graphics::DrawMode::Fill(FillOptions::DEFAULT),
                    Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                    Color::from_rgb(0, 255, 0),
                )
                .unwrap();

                (x, y) = match self.paths[y][x].unwrap().0 {
                    Dir::Up => (x, y + 1),
                    Dir::Down => (x, y - 1),
                    Dir::Left => (x + 1, y),
                    Dir::Right => (x - 1, y),
                }
            }
        } else {
            for (x, y) in self.stack.iter() {
                mb.rectangle(
                    graphics::DrawMode::Fill(FillOptions::DEFAULT),
                    Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                    Color::from_rgb(255, 255, 0),
                )
                .unwrap();
            }
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }
}

struct MainState2 {
    map: Map,
    paths: Vec<Vec<Option<(Dir, usize)>>>,
    stack: VecDeque<(usize, usize)>,
    solved: bool,
    cells_on_path: HashSet<(usize, usize)>,
}

impl event::EventHandler<ggez::GameError> for MainState2 {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if ctx.mouse.button_just_pressed(event::MouseButton::Left) {
            let x = (ctx.mouse.position().x / 5.0) as usize;
            let y = (ctx.mouse.position().y / 5.0) as usize;

            if let Some((dir, cost_from_start)) =
                find_cost(&self.map, self.map.start, Dir::Right, (x, y))
            {
                if let Some((end_dir, cost_to_end)) =
                    find_cost(&self.map, (x, y), dir, self.map.end)
                {
                    let combined = cost_from_start + cost_to_end;
                    println!("{} {} {:#?}", x, y, combined);
                }
            }
        }

        let mut max_steps = 1000;
        while let Some(next) = self.stack.pop_front() {
            max_steps -= 1;
            propagate(&self.map, next.0, next.1, &mut self.paths, &mut self.stack);

            if self.stack.is_empty() {
                self.solved = true;
                let optimal_score = self.paths[self.map.end.1][self.map.end.0].unwrap().1;

                for y in 0..self.map.height {
                    for x in 0..self.map.width {
                        if !self.map.buffer[y][x]
                            && cost_through_point(&self.map, (x, y)) == Some(optimal_score)
                        {
                            self.cells_on_path.insert((x, y));
                        }
                    }
                }

                println!("{}", self.cells_on_path.len());
            }

            if max_steps < 0 {
                return Ok(());
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        let mut mb = MeshBuilder::new();

        for (y, line) in self.map.buffer.iter().cloned().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    mb.rectangle(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                        Color::from_rgb(255, 255, 255),
                    )
                    .unwrap();
                } else if let Some((dir, score)) = self.paths[y][x] {
                    let points = match dir {
                        Dir::Down => [
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 2.5,
                                y: (y * 5) as f32 + 5.0,
                            },
                        ],
                        Dir::Up => [
                            Point2 {
                                x: (x * 5) as f32 + 2.5,
                                y: (y * 5) as f32,
                            },
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32 + 5.0,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32 + 5.0,
                            },
                        ],
                        Dir::Left => [
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32 + 2.5,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32 + 5.0,
                            },
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32,
                            },
                        ],
                        Dir::Right => [
                            Point2 {
                                x: (x * 5) as f32 + 5.0,
                                y: (y * 5) as f32 + 2.5,
                            },
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32 + 5.0,
                            },
                            Point2 {
                                x: (x * 5) as f32,
                                y: (y * 5) as f32,
                            },
                        ],
                    };

                    mb.polygon(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        &points,
                        Color::from_rgb(0, 0, 255),
                    )
                    .unwrap();
                }
            }
        }

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                (self.map.start.0 * 5) as f32,
                (self.map.start.1 * 5) as f32,
                5.0,
                5.0,
            ),
            Color::from_rgb(0, 255, 0),
        )
        .unwrap();

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                (self.map.end.0 * 5) as f32,
                (self.map.end.1 * 5) as f32,
                5.0,
                5.0,
            ),
            Color::from_rgb(255, 0, 0),
        )
        .unwrap();

        if self.solved {
            for (x, y) in self.cells_on_path.iter() {
                mb.rectangle(
                    graphics::DrawMode::Fill(FillOptions::DEFAULT),
                    Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                    Color::from_rgb(0, 255, 0),
                )
                .unwrap();
            }
        } else {
            for (x, y) in self.stack.iter() {
                mb.rectangle(
                    graphics::DrawMode::Fill(FillOptions::DEFAULT),
                    Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                    Color::from_rgb(255, 255, 0),
                )
                .unwrap();
            }
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }
}

fn find_cost(
    map: &Map,
    start: (usize, usize),
    start_dir: Dir,
    end: (usize, usize),
) -> Option<(Dir, usize)> {
    let mut stack = VecDeque::new();
    stack.push_back(start);

    let mut paths = Vec::new();
    for _ in 0..map.height {
        let mut l = Vec::new();
        for _ in 0..map.width {
            l.push(None);
        }
        paths.push(l);
    }

    paths[start.1][start.0] = Some((start_dir, 0));

    while let Some((x, y)) = stack.pop_front() {
        propagate2(map, x, y, &mut paths, &mut stack);
    }

    return paths[end.1][end.0];
}

fn propagate2(
    map: &Map,
    x: usize,
    y: usize,
    paths: &mut Vec<Vec<Option<(Dir, usize)>>>,
    stack: &mut VecDeque<(usize, usize)>,
) {
    let (dir, score) = paths[y][x].unwrap();

    // Check UP (if the last move was NOT DOWN)
    if dir != Dir::Down && y > 0 && !map.buffer[y - 1][x] {
        let next_score = if dir == Dir::Up {
            score + 1
        } else {
            score + 1001
        };
        if paths[y - 1][x].is_none() || paths[y - 1][x].unwrap().1 > next_score {
            paths[y - 1][x] = Some((Dir::Up, next_score));
            stack.push_back((x, y - 1));
        }
    }

    // Check DOWN (if the last move was NOT UP)
    if dir != Dir::Up && y < map.height - 1 && !map.buffer[y + 1][x] {
        let next_score = if dir == Dir::Down {
            score + 1
        } else {
            score + 1001
        };
        if paths[y + 1][x].is_none() || paths[y + 1][x].unwrap().1 > next_score {
            paths[y + 1][x] = Some((Dir::Down, next_score));
            stack.push_back((x, y + 1));
        }
    }

    // Check LEFT (if the last move was NOT RIGHT)
    if dir != Dir::Right && x > 0 && !map.buffer[y][x - 1] {
        let next_score = if dir == Dir::Left {
            score + 1
        } else {
            score + 1001
        };
        if paths[y][x - 1].is_none() || paths[y][x - 1].unwrap().1 > next_score {
            paths[y][x - 1] = Some((Dir::Left, next_score));
            stack.push_back((x - 1, y));
        }
    }

    // Check RIGHT (if the last move was NOT LEFT)
    if dir != Dir::Left && x < map.width - 1 && !map.buffer[y][x + 1] {
        let next_score = if dir == Dir::Right {
            score + 1
        } else {
            score + 1001
        };
        if paths[y][x + 1].is_none() || paths[y][x + 1].unwrap().1 > next_score {
            paths[y][x + 1] = Some((Dir::Right, next_score));
            stack.push_back((x + 1, y));
        }
    }
}

fn cost_through_point(map: &Map, point: (usize, usize)) -> Option<usize> {
    if let Some((dir, cost_from_start)) = find_cost(&map, map.start, Dir::Right, point) {
        if let Some((end_dir, cost_to_end)) = find_cost(&map, point, dir, map.end) {
            let combined = cost_from_start + cost_to_end;
            return Some(combined);
        }
    }
    None
}
