use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    fs::File,
    io::Read,
};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_09").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut storage = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for char in contents.chars().filter(|ch| ch.is_numeric()) {
        if is_file {
            let val = char.to_string().parse().unwrap();
            for _ in 0..val {
                storage.push(Some(file_id));
            }
            file_id += 1;
        } else {
            let val = char.to_string().parse().unwrap();
            for _ in 0..val {
                storage.push(None);
            }
        }
        is_file = !is_file;
    }

    let mut front = 0;
    let mut end = storage.len() - 1;

    while storage[end].is_none() {
        end -= 1
    }

    while front < end {
        if storage[front].is_none() {
            storage.swap(front, end);
        }
        front += 1;
        while storage[end].is_none() {
            end -= 1
        }
    }

    let sum = storage
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| match cell {
            Some(id) => Some((i, id)),
            None => None,
        })
        .map(|(i, id)| i * id)
        .sum::<usize>();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_09").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut storage = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for char in contents.chars().filter(|ch| ch.is_numeric()) {
        if is_file {
            let val = char.to_string().parse().unwrap();
            storage.push(Span {
                len: val,
                contents: Some(file_id),
            });
            file_id += 1;
        } else {
            let val = char.to_string().parse().unwrap();
            storage.push(Span {
                len: val,
                contents: None,
            });
        }
        is_file = !is_file;
    }

    for id in (0..file_id).rev() {
        println!("defragmenting {}", id);
        // let blocks = storage.iter().flat_map(|span| {
        //     let mut res = Vec::new();
        //     for _ in 0..span.len {
        //         res.push(span.contents);
        //     }
        //     res
        // });
        // print_storage(&blocks.clone().collect_vec());

        let (file_idx, file) = storage
            .iter()
            .find_position(|span| span.contents == Some(id))
            .unwrap();
        if let Some((space_idx, space)) = storage
            .iter()
            .find_position(|span| span.contents.is_none() && span.len >= file.len)
        {
            if space_idx >= file_idx {
                continue;
            }

            if space.len > file.len {
                let leftover = space.len - file.len;

                // Update the swapped space's length to match the file's length.
                storage[space_idx].len = file.len;

                // Insert the leftover space after the space that was swapped.
                storage.insert(
                    space_idx + 1,
                    Span {
                        len: leftover,
                        contents: None,
                    },
                );
                storage.swap(file_idx + 1, space_idx);
            } else {
                storage.swap(file_idx, space_idx);
            }

            storage = merge_empty_spans(storage);
        }
    }

    let blocks = storage.iter().flat_map(|span| {
        let mut res = Vec::new();
        for _ in 0..span.len {
            res.push(span.contents);
        }
        res
    });

    print_storage(&blocks.clone().collect_vec());

    let sum = blocks
        .enumerate()
        .filter_map(|(i, cell)| match cell {
            Some(id) => Some((i, id)),
            None => None,
        })
        .map(|(i, id)| i * id)
        .sum::<usize>();

    println!("{}", sum);
}

#[derive(Debug)]
struct Span {
    pub len: usize,
    pub contents: Option<usize>,
}

fn merge_empty_spans(spans: Vec<Span>) -> Vec<Span> {
    let mut result = Vec::new();
    let mut accumulated_len = 0;

    for span in spans {
        match span.contents {
            Some(_) => {
                if accumulated_len > 0 {
                    result.push(Span {
                        len: accumulated_len,
                        contents: None,
                    });
                    accumulated_len = 0;
                }
                result.push(span);
            }
            None => {
                accumulated_len += span.len;
            }
        }
    }

    result
}

fn print_storage(storage: &Vec<Option<usize>>) {
    for block in storage {
        match block {
            Some(id) => print!("{}", id % 10),
            None => print!("."),
        }
    }
    println!("");
}
