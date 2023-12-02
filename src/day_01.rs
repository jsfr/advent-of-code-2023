use anyhow::{Context, Result};
use regex::Regex;

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let calibration_values: Vec<usize> = input
            .lines()
            .map(|line| {
                let digits: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();

                let first = digits
                    .first()
                    .context(format!("no first digit present in line '{line}'"))?;
                let last = digits
                    .last()
                    .context(format!("no last digit present in line '{line}'"))?;

                let calibration_value: usize = format!("{first}{last}").parse().context(
                    format!("Failed to parse calibration value '{first}{last}' as a usize"),
                )?;

                Ok(calibration_value)
            })
            .collect::<Result<_>>()?;

        let total_value: usize = calibration_values.iter().sum();

        Ok(total_value.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let calibration_values: Vec<usize> = input
            .lines()
            .map(|line| {
                let re_str =
                    r"one|two|three|four|five|six|seven|eight|nine|zero|1|2|3|4|5|6|7|8|9|0";
                let re = Regex::new(re_str)?;

                let first = re
                    .find(line)
                    .map(|m| m.as_str())
                    .and_then(to_usize)
                    .context(format!("no first digit present in line '{line}'"))?;

                let re_str = re_str.chars().rev().collect::<String>();
                let re = Regex::new(&re_str)?;

                let reverse_line = line.to_string().chars().rev().collect::<String>();

                let last = re
                    .find(&reverse_line)
                    .map(|m| m.as_str().chars().rev().collect::<String>())
                    .and_then(|s| to_usize(&s))
                    .context(format!("no last digit present in line '{line}'"))?;

                let calibration_value: usize = first * 10 + last;

                Ok(calibration_value)
            })
            .collect::<Result<_>>()?;

        let total_value: usize = calibration_values.iter().sum();

        Ok(total_value.to_string())
    }
}

fn to_usize(s: &str) -> Option<usize> {
    match s {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}
