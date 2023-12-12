use anyhow::{Context, Result};
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let oasis_report: Vec<Vec<i64>> = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|n| n.parse().context(format!("failed to parse {n}")))
                    .collect()
            })
            .collect::<Result<_>>()?;

        let result = oasis_report
            .into_iter()
            .map(|line| {
                let mut series = vec![line];

                loop {
                    let s = series.last().unwrap();

                    if s.iter().all(|n| *n == 0) {
                        break;
                    }

                    series.push(s.iter().tuple_windows().map(|(a, b)| b - a).collect());
                }

                series
                    .into_iter()
                    .rev()
                    .fold(0, |acc, s| acc + s.last().unwrap())
            })
            .sum::<i64>();

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let oasis_report: Vec<Vec<i64>> = input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|n| n.parse().context(format!("failed to parse {n}")))
                    .collect()
            })
            .collect::<Result<_>>()?;

        let result = oasis_report
            .into_iter()
            .map(|line| {
                let mut series = vec![line];

                loop {
                    let s = series.last().unwrap();

                    if s.iter().all(|n| *n == 0) {
                        break;
                    }

                    series.push(s.iter().tuple_windows().map(|(a, b)| b - a).collect());
                }

                series
                    .into_iter()
                    .rev()
                    .fold(0, |acc, s| s.first().unwrap() - acc)
            })
            .sum::<i64>();

        Ok(result.to_string())
    }
}
