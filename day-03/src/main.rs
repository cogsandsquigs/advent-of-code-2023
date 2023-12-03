use advent_utils::macros::solution;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    part_1();
    part_2();
}

fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

#[solution(day = "03", part = "1")]
fn part_1(input: &str) -> u32 {
    0
}

#[solution(day = "03", part = "2")]
fn part_2(input: &str) -> u32 {
    0
}
