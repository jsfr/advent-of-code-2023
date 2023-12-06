use std::str::FromStr;

use anyhow::{bail, Result};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let races: Races = input.parse()?;

        let result = races
            .times
            .into_iter()
            .zip(races.records)
            .map(|(time, record)| {
                (1..time)
                    .map(|button_held| {
                        let time_remaining = time - button_held;
                        let boat_speed = button_held;

                        time_remaining * boat_speed
                    })
                    .filter(|distance| *distance > record)
                    .count()
            })
            .product::<usize>();

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let races: Races = input.parse()?;

        let time = races
            .times
            .iter()
            .fold(String::new(), |acc, t| format!("{acc}{t}"))
            .parse::<usize>()?;
        let record = races
            .records
            .iter()
            .fold(String::new(), |acc, r| format!("{acc}{r}"))
            .parse::<usize>()?;

        let result = (1..time)
            .map(|button_held| {
                let time_remaining = time - button_held;
                let boat_speed = button_held;

                time_remaining * boat_speed
            })
            .filter(|distance| *distance > record)
            .count();

        Ok(result.to_string())
    }
}

struct Races {
    times: Vec<usize>,
    records: Vec<usize>,
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match all_consuming(parse_races)(s).finish() {
            Ok((_, races)) => Ok(races),
            Err(err) => bail!("failed to parse races: '{err}'"),
        }
    }
}

fn parse_races(s: &str) -> IResult<&str, Races> {
    let mut parse_digits = separated_list1(space1, map_res(digit1, str::parse::<usize>));
    let (s, (_, _, times)) = tuple((tag("Time:"), space1, &mut parse_digits))(s)?;
    let (s, _) = newline(s)?;
    let (s, (_, _, records)) = tuple((tag("Distance:"), space1, &mut parse_digits))(s)?;
    let (s, _) = opt(newline)(s)?;

    Ok((s, Races { times, records }))
}
