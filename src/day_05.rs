use std::{
    collections::HashMap,
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    combinator::{all_consuming, map_opt, map_res, opt},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};

use crate::solution::Solution;

pub struct Day {}

impl Solution for Day {
    fn compute_1(&self, input: &str) -> Result<String> {
        use Type::{Fertilizer, Humidity, Light, Location, Seed, Soil, Temperature, Water};

        let almanac: Almanac = input.parse()?;
        let pairs = vec![
            (Seed, Soil),
            (Soil, Fertilizer),
            (Fertilizer, Water),
            (Water, Light),
            (Light, Temperature),
            (Temperature, Humidity),
            (Humidity, Location),
        ];

        let converted_seeds = pairs
            .iter()
            .map(|pair| almanac.conversion_maps.get(pair))
            .fold_options(almanac.seeds, |acc, conversion_map| {
                acc.into_iter().map(|x| conversion_map.convert(x)).collect()
            })
            .context("failed to convert from seeds to location")?;

        let result = converted_seeds
            .into_iter()
            .min()
            .context("no lowest value")?;

        Ok(result.to_string())
    }

    fn compute_2(&self, _input: &str) -> Result<String> {
        todo!()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    conversion_maps: HashMap<(Type, Type), ConversionMap>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct ConversionMap {
    entries: Vec<MapEntry>,
}

#[derive(Debug)]
struct MapEntry {
    dest: usize,
    src: usize,
    range: usize,
}

impl ConversionMap {
    fn convert(&self, from: usize) -> usize {
        if let Some(entry) = self
            .entries
            .iter()
            .find(|e| e.src <= from && from <= e.src + e.range)
        {
            entry.dest + (from - entry.src)
        } else {
            from
        }
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match all_consuming(parse_almanac)(s).finish() {
            Ok((_, almanac)) => Ok(almanac),
            Err(err) => bail!("failed to parse almanac: {err}"),
        }
    }
}

fn parse_almanac(s: &str) -> IResult<&str, Almanac> {
    let (s, (seeds, conversion_maps)) = separated_pair(
        parse_seeds,
        tag("\n\n"),
        separated_list1(tag("\n\n"), parse_conversion_map),
    )(s)?;
    let (s, _) = opt(newline)(s)?;
    let conversion_maps = conversion_maps.into_iter().collect();
    Ok((
        s,
        Almanac {
            seeds,
            conversion_maps,
        },
    ))
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<usize>> {
    let (s, _) = tag("seeds: ")(s)?;
    let (s, seeds) = separated_list1(space1, map_res(digit1, str::parse))(s)?;
    Ok((s, seeds))
}

fn parse_conversion_map(s: &str) -> IResult<&str, ((Type, Type), ConversionMap)> {
    let (s, (from, _, to, _, entries)) = tuple((
        map_opt(alpha1, get_map_type),
        tag("-to-"),
        map_opt(alpha1, get_map_type),
        tag(" map:\n"),
        separated_list1(newline, parse_map_entry),
    ))(s)?;

    Ok((s, ((from, to), ConversionMap { entries })))
}

fn parse_map_entry(s: &str) -> IResult<&str, MapEntry> {
    let mut parse_digit = map_res(digit1, str::parse);
    let (s, dest) = parse_digit(s)?;
    let (s, _) = space1(s)?;
    let (s, src) = parse_digit(s)?;
    let (s, _) = space1(s)?;
    let (s, range) = parse_digit(s)?;

    Ok((s, MapEntry { dest, src, range }))
}

fn get_map_type(s: &str) -> Option<Type> {
    let map_type = match s {
        "seed" => Type::Seed,
        "soil" => Type::Soil,
        "fertilizer" => Type::Fertilizer,
        "water" => Type::Water,
        "light" => Type::Light,
        "temperature" => Type::Temperature,
        "humidity" => Type::Humidity,
        "location" => Type::Location,
        _ => return None,
    };

    Some(map_type)
}
