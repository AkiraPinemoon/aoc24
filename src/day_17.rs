use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_17").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let registers = contents
        .lines()
        .take(3)
        .map(|line| line.split_whitespace().skip(2).next().unwrap())
        .map(|val| val.parse().unwrap())
        .collect_vec();

    let memory: Vec<Instruction> = contents
        .lines()
        .skip(4)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .filter_map(|ch| ch.parse::<u8>().ok())
        .tuples()
        .filter_map(|instruction_data: (u8, u8)| instruction_data.try_into().ok())
        .collect_vec();

    let mut machine = Machine {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        memory,
        ip: 0,
        out: Vec::new(),
    };

    machine.run();

    println!("{}", machine.out.into_iter().join(","));
}

pub fn part_two() {
    let mut file = File::open("input_17").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let rom = contents
        .lines()
        .skip(4)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .filter_map(|ch| ch.parse::<u8>().ok())
        .collect_vec();

    let res = find_input(&rom, 0);
    println!("{:#?}", res);
}

fn find_input(rom: &Vec<u8>, a: i64) -> Option<i64> {
    if rom.is_empty() {
        return Some(a >> 3);
    }

    for i in 0..8 {
        if let Some(res) = sim_prog(a + i).first() {
            if rom.last() == Some(res) {
                let mut copy = rom.clone();
                copy.pop();
                if let Some(inp) = find_input(&copy, (a + i) << 3) {
                    return Some(inp);
                }
            }
        }
    }
    None
}

fn sim_prog(mut a: i64) -> Vec<u8> {
    let mut out = Vec::new();
    while a > 0 {
        out.push(((((a % 8) ^ 5) ^ 6) ^ (a >> ((a % 8) ^ 5))) as u8 % 8);
        a = a >> 3;
    }
    out
}

struct Machine {
    a: i64,
    b: i64,
    c: i64,
    memory: Vec<Instruction>,
    ip: usize,
    out: Vec<u8>,
}

impl Machine {
    fn clock(&mut self) {
        if let Some(instruction) = self.memory.get(self.ip / 2) {
            match instruction {
                Instruction::ADV(combo_op) => {
                    self.a = self.a >> self.get_combo(combo_op);
                    self.ip += 2;
                }
                Instruction::BXL(literal_op) => {
                    self.b = self.b ^ literal_op.0 as i64;
                    self.ip += 2;
                }
                Instruction::BST(combo_op) => {
                    self.b = (self.get_combo(combo_op) % 8) as i64;
                    self.ip += 2;
                }
                Instruction::JNZ(literal_op) => {
                    if self.a == 0 {
                        self.ip += 2;
                    } else {
                        self.ip = literal_op.0 as usize;
                    }
                }
                Instruction::BXC => {
                    self.b = self.b ^ self.c;
                    self.ip += 2;
                }
                Instruction::OUT(combo_op) => {
                    self.out.push((self.get_combo(combo_op) % 8) as u8);
                    self.ip += 2;
                }
                Instruction::BDV(combo_op) => {
                    self.b = self.a >> self.get_combo(combo_op);
                    self.ip += 2;
                }
                Instruction::CDV(combo_op) => {
                    self.c = self.a >> self.get_combo(combo_op);
                    self.ip += 2;
                }
            }
        }
    }

    fn get_combo(&self, op: &ComboOP) -> i64 {
        match op {
            ComboOP::Val0 => 0,
            ComboOP::Val1 => 1,
            ComboOP::Val2 => 2,
            ComboOP::Val3 => 3,
            ComboOP::RegA => self.a,
            ComboOP::RegB => self.b,
            ComboOP::RegC => self.c,
        }
    }

    fn run(&mut self) {
        while self.ip / 2 < self.memory.len() {
            self.clock();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ADV(ComboOP),
    BXL(LiteralOP),
    BST(ComboOP),
    JNZ(LiteralOP),
    BXC,
    OUT(ComboOP),
    BDV(ComboOP),
    CDV(ComboOP),
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = ();

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let (operator, operand) = value;
        match operator {
            0 => Ok(Self::ADV(operand.try_into()?)),
            1 => Ok(Self::BXL(operand.into())),
            2 => Ok(Self::BST(operand.try_into()?)),
            3 => Ok(Self::JNZ(operand.into())),
            4 => Ok(Self::BXC),
            5 => Ok(Self::OUT(operand.try_into()?)),
            6 => Ok(Self::BDV(operand.try_into()?)),
            7 => Ok(Self::CDV(operand.try_into()?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ComboOP {
    Val0,
    Val1,
    Val2,
    Val3,
    RegA,
    RegB,
    RegC,
}

impl TryFrom<u8> for ComboOP {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Val0),
            1 => Ok(Self::Val1),
            2 => Ok(Self::Val2),
            3 => Ok(Self::Val3),
            4 => Ok(Self::RegA),
            5 => Ok(Self::RegB),
            6 => Ok(Self::RegC),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LiteralOP(u8);

impl From<u8> for LiteralOP {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
