use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, alphanumeric1},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let (input_instr, input_graph) =
            input.split_once("\n\n").context("could not split input")?;

        let instrs = input_instr
            .chars()
            .filter_map(|c| match c {
                'L' => Some(Instr::Left),
                'R' => Some(Instr::Right),
                _ => None,
            })
            .collect_vec();

        let Ok((_, graph)) = all_consuming(parse_graph)(input_graph).finish() else {
            bail!("failed to parse graph")
        };

        let mut current_node = "AAA";
        let mut current_instr = 0;
        let mut total_steps = 0;

        while current_node != "ZZZ" {
            let instr = &instrs[current_instr];

            current_node = match instr {
                Instr::Left => graph[current_node].0,
                Instr::Right => graph[current_node].1,
            };

            current_instr += 1;
            current_instr %= instrs.len();

            total_steps += 1;
        }

        Ok(total_steps.to_string())
    }

    fn compute_2(&self, _input: &str) -> Result<String> {
        todo!()
    }
}

enum Instr {
    Left,
    Right,
}

fn parse_graph(s: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    let (s, nodes) = separated_list1(newline, parse_node)(s)?;
    let (s, _) = opt(newline)(s)?;

    Ok((s, nodes.into_iter().collect()))
}

fn parse_node(s: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (s, node) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(s)?;
    Ok((s, node))
}
