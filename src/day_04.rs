use std::str::FromStr;

use anyhow::{bail, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{all_consuming, map_res},
    multi::many1,
    sequence::separated_pair,
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let cards: Vec<Card> = input
            .lines()
            .map(str::parse::<Card>)
            .collect::<Result<_>>()?;

        let scores = cards.iter().map(Card::calculate_score).collect_vec();

        let result = scores.iter().sum::<usize>();

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let cards: Vec<Card> = input
            .lines()
            .map(str::parse::<Card>)
            .collect::<Result<_>>()?;

        let mut number_of_cards = vec![1; cards.len()];

        for (i, card) in cards.into_iter().enumerate() {
            let score = card.calculate_matching_numbers();
            for j in i + 1..i + 1 + score {
                number_of_cards[j] += number_of_cards[i]
            }
        }

        let result = number_of_cards.iter().sum::<usize>();

        Ok(result.to_string())
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

impl Card {
    fn calculate_matching_numbers(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }

    fn calculate_score(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .fold(0, |acc, _| if acc == 0 { 1 } else { 2 * acc })
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match all_consuming(parse_card)(s).finish() {
            Ok((_, card)) => Ok(card),
            Err(err) => bail!("failed to parse card '{s}' with error: {err}"),
        }
    }
}

fn parse_number(s: &str) -> IResult<&str, usize> {
    let (s, _) = space0(s)?;
    let (s, number) = map_res(digit1, str::parse::<usize>)(s)?;
    let (s, _) = space0(s)?;

    Ok((s, number))
}

fn parse_number_list(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, _) = space0(s)?;
    let (s, numbers) = many1(parse_number)(s)?;
    let (s, _) = space0(s)?;

    Ok((s, numbers))
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, _) = tag("Card")(s)?;
    let (s, _) = space0(s)?;
    let (s, id) = map_res(digit1, str::parse::<usize>)(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space0(s)?;
    let (s, (winning_numbers, card_numbers)) =
        separated_pair(parse_number_list, tag("|"), parse_number_list)(s)?;

    Ok((
        s,
        Card {
            id,
            winning_numbers,
            card_numbers,
        },
    ))
}
