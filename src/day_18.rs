use std::{collections::VecDeque, fs::File, io::Read, usize};

use ggez::{
    conf::WindowMode,
    event,
    graphics::{self, Color, DrawParam, FillOptions, Mesh, MeshBuilder, Rect},
    ContextBuilder,
};
use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_18").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let size = 71;

    let mut map = (0..size)
        .map(|_| (0..size).map(|_| false).collect_vec())
        .collect_vec();

    for (x, y) in contents
        .lines()
        .map(|line| {
            let mut sp = line.split(',');
            let x: usize = sp.next().unwrap().parse().unwrap();
            let y: usize = sp.next().unwrap().parse().unwrap();
            (x, y)
        })
        .take(1024)
    {
        map[y][x] = true;
    }

    let mut paths = (0..size)
        .map(|_| (0..size).map(|_| None).collect_vec())
        .collect_vec();

    paths[0][0] = Some(0);

    let stack = VecDeque::from([(0, 0)]);

    let cb = ContextBuilder::new("day 16", "akira").window_mode(WindowMode::dimensions(
        WindowMode::default(),
        (size * 5) as f32,
        (size * 5) as f32,
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
    let mut file = File::open("input_18").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let size = 71;

    let mut map = (0..size)
        .map(|_| (0..size).map(|_| false).collect_vec())
        .collect_vec();

    let mut bytes = contents
        .lines()
        .map(|line| {
            let mut sp = line.split(',');
            let x: usize = sp.next().unwrap().parse().unwrap();
            let y: usize = sp.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect_vec();

    for _ in 0..1000 {
        let (x, y) = bytes.remove(0);
        map[y][x] = true;
    }

    let mut paths = (0..size)
        .map(|_| (0..size).map(|_| None).collect_vec())
        .collect_vec();

    paths[0][0] = Some(0);

    let stack = VecDeque::from([(0, 0)]);

    let cb = ContextBuilder::new("day 16", "akira").window_mode(WindowMode::dimensions(
        WindowMode::default(),
        (size * 5) as f32,
        (size * 5) as f32,
    ));
    let (ctx, event_loop) = cb.build().unwrap();
    let state = MainState2 {
        map,
        paths,
        stack,
        solved: false,
        bytes,
        current: (0, 0),
    };
    event::run(ctx, event_loop, state);
}

fn propagate(
    map: &Vec<Vec<bool>>,
    x: usize,
    y: usize,
    paths: &mut Vec<Vec<Option<usize>>>,
    stack: &mut VecDeque<(usize, usize)>,
) {
    let score = paths[y][x].unwrap();

    // Check UP
    if y > 0 && !map[y - 1][x] {
        if paths[y - 1][x].is_none() || paths[y - 1][x].unwrap() > score + 1 {
            paths[y - 1][x] = Some(score + 1);
            stack.push_back((x, y - 1));
        }
    }

    // Check DOWN
    if y < map.len() - 1 && !map[y + 1][x] {
        if paths[y + 1][x].is_none() || paths[y + 1][x].unwrap() > score + 1 {
            paths[y + 1][x] = Some(score + 1);
            stack.push_back((x, y + 1));
        }
    }

    // Check LEFT
    if x > 0 && !map[y][x - 1] {
        if paths[y][x - 1].is_none() || paths[y][x - 1].unwrap() > score + 1 {
            paths[y][x - 1] = Some(score + 1);
            stack.push_back((x - 1, y));
        }
    }

    // Check RIGHT
    if x < map.len() - 1 && !map[y][x + 1] {
        if paths[y][x + 1].is_none() || paths[y][x + 1].unwrap() > score + 1 {
            paths[y][x + 1] = Some(score + 1);
            stack.push_back((x + 1, y));
        }
    }
}

struct MainState {
    map: Vec<Vec<bool>>,
    paths: Vec<Vec<Option<usize>>>,
    stack: VecDeque<(usize, usize)>,
    solved: bool,
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut max_steps = 5;
        while let Some(next) = self.stack.pop_front() {
            max_steps -= 1;
            propagate(&self.map, next.0, next.1, &mut self.paths, &mut self.stack);

            if self.stack.is_empty() {
                self.solved = true;
                let score = self.paths[self.map.len() - 1][self.map.len() - 1].unwrap();
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

        for (y, line) in self.map.iter().cloned().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    mb.rectangle(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                        Color::from_rgb(255, 255, 255),
                    )
                    .unwrap();
                } else if let Some(score) = self.paths[y][x] {
                    mb.rectangle(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                        Color::from_rgb(0, 0, 255),
                    )
                    .unwrap();
                }
            }
        }

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(0.0, 0.0, 5.0, 5.0),
            Color::from_rgb(0, 255, 0),
        )
        .unwrap();

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                (self.map.len() * 5) as f32,
                (self.map.len() * 5) as f32,
                5.0,
                5.0,
            ),
            Color::from_rgb(255, 0, 0),
        )
        .unwrap();

        for (x, y) in self.stack.iter() {
            mb.rectangle(
                graphics::DrawMode::Fill(FillOptions::DEFAULT),
                Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                Color::from_rgb(255, 255, 0),
            )
            .unwrap();
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }
}

struct MainState2 {
    map: Vec<Vec<bool>>,
    paths: Vec<Vec<Option<usize>>>,
    stack: VecDeque<(usize, usize)>,
    solved: bool,
    bytes: Vec<(usize, usize)>,
    current: (usize, usize),
}

impl event::EventHandler<ggez::GameError> for MainState2 {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut max_steps = 5000;
        while let Some(next) = self.stack.pop_front() {
            max_steps -= 1;
            propagate(&self.map, next.0, next.1, &mut self.paths, &mut self.stack);

            if self.stack.is_empty() {
                self.solved = self.paths[self.map.len() - 1][self.map.len() - 1].is_some();

                if self.solved {
                    let (x, y) = self.bytes.remove(0);
                    self.current = (x, y);
                    self.map[y][x] = true;

                    self.paths = (0..self.map.len())
                        .map(|_| (0..self.map.len()).map(|_| None).collect_vec())
                        .collect_vec();

                    self.paths[0][0] = Some(0);

                    self.stack = VecDeque::from([(0, 0)]);
                } else {
                    println!("failed on {},{}", self.current.0, self.current.1);
                }
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

        for (y, line) in self.map.iter().cloned().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    mb.rectangle(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                        Color::from_rgb(255, 255, 255),
                    )
                    .unwrap();
                } else if let Some(score) = self.paths[y][x] {
                    mb.rectangle(
                        graphics::DrawMode::Fill(FillOptions::DEFAULT),
                        Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                        Color::from_rgb(0, 0, 255),
                    )
                    .unwrap();
                }
            }
        }

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(0.0, 0.0, 5.0, 5.0),
            Color::from_rgb(0, 255, 0),
        )
        .unwrap();

        mb.rectangle(
            graphics::DrawMode::Fill(FillOptions::DEFAULT),
            Rect::new(
                (self.map.len() * 5) as f32,
                (self.map.len() * 5) as f32,
                5.0,
                5.0,
            ),
            Color::from_rgb(255, 0, 0),
        )
        .unwrap();

        for (x, y) in self.stack.iter() {
            mb.rectangle(
                graphics::DrawMode::Fill(FillOptions::DEFAULT),
                Rect::new((x * 5) as f32, (y * 5) as f32, 5.0, 5.0),
                Color::from_rgb(255, 255, 0),
            )
            .unwrap();
        }

        let mesh = Mesh::from_data(ctx, mb.build());
        canvas.draw(&mesh, DrawParam::new());
        canvas.finish(ctx)?;

        Ok(())
    }
}