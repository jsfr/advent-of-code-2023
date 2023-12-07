use core::panic;
use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Result};
use itertools::Itertools;
use nom::{
    character::complete::{digit1, newline, one_of, space1},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        let mut game: GameNoJokers = input.parse()?;

        game.hands_and_bids.sort_by_key(|(hand, _)| *hand);
        game.hands_and_bids.reverse();

        game.hands_and_bids.iter().for_each(|(h, _)| {
            println!("Hand '{:?}' is '{:?}'", h.cards, h.hand_type());
        });

        let result =
            game.hands_and_bids
                .into_iter()
                .enumerate()
                .fold(0_usize, |total, (idx, (_, bid))| {
                    let rank = idx + 1;
                    let score = rank * bid;
                    total + score
                });

        Ok(result.to_string())
    }

    fn compute_2(&self, input: &str) -> Result<String> {
        let mut game: GameWithJokers = input.parse()?;

        game.hands_and_bids.sort_by_key(|(hand, _)| *hand);
        game.hands_and_bids.reverse();

        let result =
            game.hands_and_bids
                .into_iter()
                .enumerate()
                .fold(0_usize, |total, (idx, (_, bid))| {
                    let rank = idx + 1;
                    let score = rank * bid;
                    total + score
                });

        Ok(result.to_string())
    }
}

#[derive(Debug)]
struct GameNoJokers {
    hands_and_bids: Vec<(Hand, usize)>,
}

#[derive(Debug)]
struct GameWithJokers {
    hands_and_bids: Vec<(Hand, usize)>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: (Card, Card, Card, Card, Card),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl FromStr for GameNoJokers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match all_consuming(parse_hands_and_bids(false))(s).finish() {
            Ok((_, hands_and_bids)) => Ok(Self { hands_and_bids }),
            Err(err) => bail!("failed to parse game: {err}"),
        }
    }
}

impl FromStr for GameWithJokers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match all_consuming(parse_hands_and_bids(true))(s).finish() {
            Ok((_, hands_and_bids)) => Ok(Self { hands_and_bids }),
            Err(err) => bail!("failed to parse game: {err}"),
        }
    }
}

fn parse_hands_and_bids(use_jokers: bool) -> impl Fn(&str) -> IResult<&str, Vec<(Hand, usize)>> {
    move |s: &str| {
        let parse_hand = parse_hand(use_jokers);
        let (s, hands_and_bids) = separated_list1(
            newline,
            separated_pair(parse_hand, space1, map_res(digit1, str::parse::<usize>)),
        )(s)?;
        let (s, _) = opt(newline)(s)?;

        Ok((s, hands_and_bids))
    }
}

fn parse_hand(use_jokers: bool) -> impl Fn(&str) -> IResult<&str, Hand> {
    move |s: &str| {
        let parse_card = parse_card(use_jokers);
        let (s, cards) = tuple((
            &parse_card,
            &parse_card,
            &parse_card,
            &parse_card,
            &parse_card,
        ))(s)?;
        Ok((s, Hand { cards }))
    }
}

fn parse_card(use_jokers: bool) -> impl Fn(&str) -> IResult<&str, Card> {
    move |s: &str| {
        use Card::{Ace, Eight, Five, Four, Jack, Joker, King, Nine, Queen, Seven, Six, Ten, Three, Two};

        let (s, card) = one_of("AKQJT98765432")(s)?;
        let card = match card {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => {
                if use_jokers {
                    Joker
                } else {
                    Jack
                }
            }
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => panic!("this can't happen"),
        };

        Ok((s, card))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_self = self.hand_type();
        let hand_type_other = other.hand_type();

        match hand_type_self.cmp(&hand_type_other) {
            std::cmp::Ordering::Equal => {
                // Break the tie
                self.cards.cmp(&other.cards)
            }
            ordering => ordering,
        }
    }
}

impl Hand {
    fn hand_type(self) -> HandType {
        use HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs};

        if self.has_n_of_a_kind(5) {
            FiveOfAKind
        } else if self.has_n_of_a_kind(4) {
            FourOfAKind
        } else if self.has_full_house() {
            FullHouse
        } else if self.has_n_of_a_kind(3) {
            ThreeOfAKind
        } else if self.has_two_pairs() {
            TwoPairs
        } else if self.has_one_pair() {
            OnePair
        } else {
            HighCard
        }
    }

    fn has_n_of_a_kind(self, n: usize) -> bool {
        let mut counts = self.counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let max = counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map_or(0, |(_, count)| count);

        max + jokers == n
    }

    fn has_full_house(self) -> bool {
        let mut counts = self.counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);

        if let Some((c1, c2)) = counts.values().take(2).collect_tuple() {
            c1 + c2 + jokers == 5
        } else {
            false
        }
    }

    fn has_two_pairs(self) -> bool {
        let mut counts = self.counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let pair = counts
            .into_iter()
            .sorted_by_key(|(_, count)| *count)
            .rev()
            .map(|(_, count)| count)
            .take(2)
            .collect_tuple();

        if let Some((c1, c2)) = pair {
            c1 + c2 + jokers == 4
        } else {
            false
        }
    }

    fn has_one_pair(self) -> bool {
        self.has_n_of_a_kind(2)
    }

    fn counts(self) -> HashMap<Card, usize> {
        vec![
            self.cards.0,
            self.cards.1,
            self.cards.2,
            self.cards.3,
            self.cards.4,
        ]
        .into_iter()
        .counts()
    }
}
