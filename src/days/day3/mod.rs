use itertools::Itertools;

use super::get_input;

/**
 * --- Day 3 ---
 *
 * https://adventofcode.com/2022/day/3
 */
pub fn solve() {
    println!("Day 3 :");
    part_one();
    part_two();
}

fn part_one() {
    let input = get_input(3);
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (first_part, second_part) = line.split_at(line.len() / 2);
            let map = first_part
                .chars()
                .unique()
                .chain(second_part.chars().unique())
                .counts();

            let (key, _) = map.iter().find(|kv| *kv.1 == 2).unwrap();
            (match key.is_uppercase() {
                true => key.to_digit(36).unwrap() + 26,
                false => key.to_digit(36).unwrap(),
            } - 9)
        })
        .sum();

    println!("priority {sum}")
}

fn part_two() {
    let input = get_input(3);
    let lines = input.lines();

    let sum = (&lines.chunks(3))
        .into_iter()
        .map(|mut chunk| {
            let (a, b, c) = chunk.next_tuple().unwrap();
            let map = a
                .chars()
                .unique()
                .chain(b.chars().unique())
                .chain(c.chars().unique())
                .counts();

            let (key, _) = map.iter().find(|kv| *kv.1 == 3).unwrap();
            (match key.is_uppercase() {
                true => key.to_digit(36).unwrap() + 26,
                false => key.to_digit(36).unwrap(),
            } - 9)
        })
        .sum::<u32>();
    println!("priority {sum}")
}
