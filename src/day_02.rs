use std::{cmp, str::FromStr};

use anyhow::{bail, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until1},
    character::complete::{alpha1, digit1, line_ending, multispace0},
    combinator::{all_consuming, map_res},
    multi::{many1, many_till, separated_list1},
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(Debug, Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn calculate_power(self: &Self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn calculate_min_set(self: &Self) -> Set {
        self.sets
            .iter()
            .fold(Set::default(), |mut min_set, curr_set| {
                min_set.red = cmp::max(min_set.red, curr_set.red);
                min_set.green = cmp::max(min_set.green, curr_set.green);
                min_set.blue = cmp::max(min_set.blue, curr_set.blue);

                min_set
            })
    }
}

fn parse_color(input: &str) -> IResult<&str, (usize, &str)> {
    let (input, n) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, color) = alpha1(input)?;

    Ok((input, (n, color)))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let mut set = Set::default();
    let (input, colors) = separated_list1(tag(", "), parse_color)(input)?;

    for (n, color) in colors {
        match color {
            "red" => set.red = n,
            "green" => set.green = n,
            "blue" => set.blue = n,
            _ => (),
        }
    }

    Ok((input, set))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = is_a(": ")(input)?;
    let (input, sets) = separated_list1(tag("; "), parse_set)(input)?;

    Ok((input, Game { id, sets }))
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match all_consuming(parse_game)(s).finish() {
            Ok((_, game)) => Ok(game),
            Err(err) => bail!("failed to parse line '{s}' with error: {err}"),
        }
    }
}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let games: Vec<Game> = input.lines().map(str::parse).collect::<Result<_>>()?;

        let (max_red, max_green, max_blue) = (12_usize, 13_usize, 14_usize);

        let possible_games = games
            .into_iter()
            .filter(|g| {
                g.sets
                    .iter()
                    .all(|s| s.red <= max_red && s.green <= max_green && s.blue <= max_blue)
            })
            .collect_vec();

        let result = possible_games.iter().map(|g| g.id).sum::<usize>();

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let games: Vec<Game> = input.lines().map(str::parse).collect::<Result<_>>()?;

        let min_sets = games.into_iter().map(|g| g.calculate_min_set());

        let powers = min_sets.map(|s| s.calculate_power());

        let result = powers.sum::<usize>();

        Ok(result.to_string())
    }
}
