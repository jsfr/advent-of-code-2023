use std::{str::FromStr, fmt::Display};

use anyhow::{Result, bail};
use itertools::Itertools;
use ndarray::Array2;

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let universe: Universe = input.parse()?;
        let universe = universe.expand();
        let galaxies = universe.get_galaxies();

        let mut result = 0_usize;
        for i in 0..galaxies.len()-1 {
            for j in (i+1)..galaxies.len() {
                result += galaxies[i].distance(&galaxies[j]);
            }
        }

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let universe: Universe = input.parse()?;
        let galaxies = universe.get_galaxies();

        let mut before_expansion = 0_usize;
        for i in 0..galaxies.len()-1 {
            for j in (i+1)..galaxies.len() {
                before_expansion += galaxies[i].distance(&galaxies[j]);
            }
        }

        let universe = universe.expand();
        let galaxies = universe.get_galaxies();

        let mut after_expansion = 0_usize;
        for i in 0..galaxies.len()-1 {
            for j in (i+1)..galaxies.len() {
                after_expansion += galaxies[i].distance(&galaxies[j]);
            }
        }

        let one_expansion_diff = after_expansion - before_expansion;

        let result = before_expansion + (one_expansion_diff * 999_999);

        Ok(result.to_string())
    }
}

struct Universe {
    universe: Array2<Space>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);

        x + y
    }
}

#[derive(Clone, Copy)]
enum Space {
    Empty,
    Galaxy,
}

impl Universe {
    fn expand(self) -> Self {
        let mut rows = Vec::with_capacity(self.universe.shape()[0]);

        for row in self.universe.rows() {
            if row.iter().all(|space| matches!(space, Space::Empty)) {
                rows.push(vec![Space::Empty; row.len()]);
            }
            rows.push(row.to_vec())
        }

        let universe = Array2::from_shape_fn((rows.len(), rows[0].len()), |(y,x)| {
            rows[y][x]
        });

        let mut columns = Vec::with_capacity(self.universe.shape()[1]);

        for column in universe.columns() {
            if column.iter().all(|space| matches!(space, Space::Empty)) {
                columns.push(vec![Space::Empty; column.len()])
            }
            columns.push(column.to_vec())
        }

        let universe = Array2::from_shape_fn((columns[0].len(), columns.len()), |(y, x)| {
            columns[x][y]
        });

        Self {
            universe
        }
    }

    fn get_galaxies(&self) -> Vec<Point> {
        let mut galaxies = vec![];
        let shape = self.universe.shape();

        for y in 0..shape[0] {
            for x in 0..shape[1] {
                let space = self.universe.get((y,x)).unwrap();
                if matches!(space, Space::Galaxy) {
                    galaxies.push(Point {x, y})
                }
            }
        }

        galaxies
    }
}

impl FromStr for Universe {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let u_vec: Vec<Vec<_>> = s.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                   '#' => Ok(Space::Galaxy),
                    '.' => Ok(Space::Empty),
                    _ => bail!("Unknown space type '{c}'")
                }
            }).collect()
        }).collect::<Result<_>>()?;

        let universe = Array2::from_shape_fn((u_vec.len(), u_vec[0].len()), |(y, x)| {
            u_vec[y][x]
        });

        Ok(Self { universe })
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.universe.rows() {
            row.iter().for_each(|space| {
                let _ = write!(f, "{}", match space {
                    Space::Empty => ".",
                    Space::Galaxy => "#",
                });
            });
            let _ = write!(f, "\n");
        }

        Ok(())
    }
}
