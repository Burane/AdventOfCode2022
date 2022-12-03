use super::get_input;
use std::{ops::Add, str::FromStr};

/**
 * --- Day 2 ---
 *
 * https://adventofcode.com/2022/day/2
 */

pub fn solve() {
    part_one();
    part_two();
}

#[derive(Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum MatchResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Add for Choice {
    type Output = u32;
    fn add(self, rhs: Self) -> Self::Output {
        rhs as u32
            + match (rhs, self) {
                (Choice::Rock, Choice::Rock) => MatchResult::Draw as u32,
                (Choice::Rock, Choice::Paper) => MatchResult::Loss as u32,
                (Choice::Rock, Choice::Scissors) => MatchResult::Win as u32,
                (Choice::Paper, Choice::Rock) => MatchResult::Win as u32,
                (Choice::Paper, Choice::Paper) => MatchResult::Draw as u32,
                (Choice::Paper, Choice::Scissors) => MatchResult::Loss as u32,
                (Choice::Scissors, Choice::Rock) => MatchResult::Loss as u32,
                (Choice::Scissors, Choice::Paper) => MatchResult::Win as u32,
                (Choice::Scissors, Choice::Scissors) => MatchResult::Draw as u32,
            }
    }
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(string: &str) -> Result<Choice, Self::Err> {
        match string {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for MatchResult {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "X" => Ok(MatchResult::Loss),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(()),
        }
    }
}

fn part_one() {
    let input = get_input(2);
    let points: u32 = input
        .lines()
        .map(|line| {
            let mut choices = line
                .split_whitespace()
                .map(|char| Choice::from_str(char).unwrap());
            choices.next().unwrap() + choices.next().unwrap()
        })
        .sum();

    println!("I have {points} points")
}

fn part_two() {
    let input = get_input(2);
    let points: u32 = input
        .lines()
        .map(|line| {
            let mut choices = line.split_whitespace();
            let oponent_choice = Choice::from_str(choices.next().unwrap()).unwrap();
            let match_result = MatchResult::from_str(choices.next().unwrap()).unwrap();
            let my_choice = match (oponent_choice, match_result) {
                (Choice::Rock, MatchResult::Win) => Choice::Paper,
                (Choice::Rock, MatchResult::Draw) => Choice::Rock,
                (Choice::Rock, MatchResult::Loss) => Choice::Scissors,
                (Choice::Paper, MatchResult::Win) => Choice::Scissors,
                (Choice::Paper, MatchResult::Draw) => Choice::Paper,
                (Choice::Paper, MatchResult::Loss) => Choice::Rock,
                (Choice::Scissors, MatchResult::Win) => Choice::Rock,
                (Choice::Scissors, MatchResult::Draw) => Choice::Scissors,
                (Choice::Scissors, MatchResult::Loss) => Choice::Paper,
            };
            oponent_choice + my_choice
        })
        .sum();

    println!("I have {points} points")
}
