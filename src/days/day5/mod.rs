use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use super::get_input;

/**²
 * --- Day 5: ---
 *
 * https://adventofcode.com/2022/day/5
 */
pub fn solve() {
    println!("Day 5 :");
    part_one();
    part_two();
}

fn part_one() {
    let input = get_input(5);
    let (towers_str, moves) = input.split_once("\n\n").unwrap();

    let mut towers: HashMap<usize, VecDeque<char>> = HashMap::new();

    towers_str.lines().for_each(|line| {
        let mut line_str = line.to_string();
        line_str.push(' ');
        let chars = line_str.chars();

        (&chars.chunks(4))
            .into_iter()
            .enumerate()
            .for_each(|(i, mut chunk)| {
                let (_, letter, _, _) = chunk.next_tuple().unwrap();
                if letter.is_alphabetic() {
                    towers
                        .entry(i)
                        .or_insert_with(VecDeque::new)
                        .push_front(letter);
                }
            });
    });

    moves.lines().for_each(|line| {
        let (m, f, t) = line
            .split_whitespace()
            .filter_map(|chunk| chunk.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();

        for _ in 0..m {
            let letter = towers.get_mut(&(f - 1)).unwrap().pop_back().unwrap();
            towers.get_mut(&(t - 1)).unwrap().push_back(letter)
        }
    });

    let top_chars : Vec<char> = towers
        .into_iter()
        .sorted()
        .map(|(_, tower)| tower.back().unwrap().to_owned())
        .collect();
    
        for c in top_chars {
            print!("{c}");
        }
        print!("\n")

}

fn part_two() {}
