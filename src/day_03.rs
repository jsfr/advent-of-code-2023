use anyhow::Result;
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let fields = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Field::Empty,
                        c if c.is_ascii_digit() => Field::Number(c),
                        c => Field::Symbol(c),
                    })
                    .collect_vec()
            })
            .collect_vec();

        let mut part_number = String::new();
        let mut part_numbers: Vec<usize> = Vec::new();
        let mut is_part = false;

        for i in 0..fields.len() {
            for j in 0..fields[i].len() {
                match fields[i][j] {
                    Field::Empty => {
                        if part_number.is_empty() || !is_part {
                            let above = if i == 0 {
                                false
                            } else {
                                fields
                                    .get(i - 1)
                                    .and_then(|line| line.get(j))
                                    .map_or(false, is_symbol)
                            };
                            let below = fields
                                .get(i + 1)
                                .and_then(|line| line.get(j))
                                .map_or(false, is_symbol);
                            is_part = above || below;
                        }

                        if part_number.len() > 0 {
                            if is_part {
                                part_numbers.push(part_number.parse().unwrap())
                            }
                            part_number.clear();
                        }
                    }
                    Field::Number(c) => {
                        part_number.push(c);
                        if !is_part {
                            let above = if i == 0 {
                                false
                            } else {
                                fields
                                    .get(i - 1)
                                    .and_then(|line| line.get(j))
                                    .map_or(false, is_symbol)
                            };
                            let below = fields
                                .get(i + 1)
                                .and_then(|line| line.get(j))
                                .map_or(false, is_symbol);
                            is_part = above || below;
                        }
                    }
                    Field::Symbol(_) => {
                        is_part = true;

                        if part_number.len() > 0 {
                            if is_part {
                                part_numbers.push(part_number.parse().unwrap())
                            }
                            part_number.clear();
                        }
                    }
                }
            }
        }

        let result: usize = part_numbers.into_iter().sum();

        Ok(result.to_string())
    }

    fn compute_2(&self, _input: &str) -> Result<String> {
        todo!()
    }
}

fn is_symbol(f: &Field) -> bool {
    match f {
        Field::Symbol(_) => true,
        _ => false,
    }
}

#[derive(Debug)]
enum Field {
    Empty,
    Number(char),
    Symbol(char),
}
