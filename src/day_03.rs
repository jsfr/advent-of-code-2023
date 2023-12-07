use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let map: Map = input.parse()?;

        let parts = map.get_parts();

        let result: usize = parts.into_iter().sum();

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let map: Map = input.parse()?;

        let gear_parts = map.get_gear_parts();

        let result: usize = gear_parts.into_iter().map(|(a, b)| a * b).sum();

        Ok(result.to_string())
    }
}

#[derive(Debug)]
enum Type {
    Empty,
    Digit(usize),
    Symbol(char),
}

struct Map {
    grid: Vec<Vec<Type>>,
    numbers: Vec<(usize, Vec<Pos>)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Pos {
    y: usize,
    x: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Type::{Digit, Empty, Symbol};

        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Empty,
                        d if d.is_ascii_digit() => Digit(d.to_digit(10).unwrap() as usize),
                        s => Symbol(s),
                    })
                    .collect_vec()
            })
            .collect_vec();

        let mut numbers: Vec<(usize, Vec<Pos>)> = Vec::new();

        for (y, line) in grid.iter().enumerate() {
            let mut tmp_pos: Vec<Pos> = vec![];
            let mut tmp_number: Vec<usize> = vec![];

            for (x, val) in line.iter().enumerate() {
                match val {
                    Digit(d) => {
                        tmp_number.push(*d);
                        tmp_pos.push(Pos { y, x });
                        if x == line.len() - 1 && !tmp_pos.is_empty() && !tmp_number.is_empty() {
                            let number = tmp_number.iter().join("").parse().unwrap();
                            let pos = tmp_pos.iter().copied().collect_vec();

                            tmp_pos.clear();
                            tmp_number.clear();

                            numbers.push((number, pos));
                        }
                    }
                    _ => {
                        if !tmp_pos.is_empty() && !tmp_number.is_empty() {
                            let number = tmp_number.iter().join("").parse().unwrap();
                            let pos = tmp_pos.iter().copied().collect_vec();

                            tmp_pos.clear();
                            tmp_number.clear();

                            numbers.push((number, pos));
                        }
                    }
                }
            }
        }

        Ok(Map { grid, numbers })
    }
}

impl Map {
    fn get_entry(&self, pos: &Pos) -> Option<&Type> {
        self.grid.get(pos.y).and_then(|line| line.get(pos.x))
    }

    fn contains_symbol(&self, positions: Vec<Pos>) -> bool {
        positions.iter().any(|pos| {
            if let Some(t) = self.get_entry(pos) {
                match t {
                    Type::Symbol(_) => true,
                    _ => false,
                }
            } else {
                false
            }
        })
    }

    fn get_parts(&self) -> Vec<usize> {
        self.numbers
            .iter()
            .filter_map(|(number, positions)| {
                let neighbours = self.calculate_neighbours(positions);
                if self.contains_symbol(neighbours) {
                    Some(*number)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_gear_parts(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, t)| match t {
                    Type::Symbol('*') => Some(Pos { y, x }),
                    _ => None,
                })
            })
            .filter_map(|position| {
                let neighbours = self.calculate_neighbours(&vec![position]);
                let adjacent_numbers = self
                    .numbers
                    .iter()
                    .filter(|(_, number_positions)| {
                        neighbours
                            .iter()
                            .any(|neighbour| number_positions.contains(neighbour))
                    })
                    .map(|(number, _)| number)
                    .collect_vec();

                if adjacent_numbers.len() == 2 {
                    Some((*adjacent_numbers[0], *adjacent_numbers[1]))
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn calculate_neighbours(&self, number_positions: &Vec<Pos>) -> Vec<Pos> {
        let diffs = [
            (-1, 0),
            (-1, -1),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        number_positions
            .iter()
            .flat_map(|pos| {
                diffs
                    .iter()
                    .map(|diff| {
                        let y = pos.y as i32 + diff.0;
                        let x = pos.x as i32 + diff.1;

                        if x >= 0
                            && y >= 0
                            && x < self.grid[0].len() as i32
                            && y < self.grid.len() as i32
                        {
                            Some(Pos {
                                y: y as usize,
                                x: x as usize,
                            })
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .flatten()
            .filter(|pos| !number_positions.contains(pos))
            .unique()
            .sorted()
            .collect()
    }
}
