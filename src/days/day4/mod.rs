use super::get_input;

/**
 * --- Day 4: ---
 *
 * https://adventofcode.com/2022/day/4
 */
pub fn solve() {
    println!("Day 4 :");
    part_one();
    part_two();
}

fn part_one() {
    let input = get_input(4);
    let sum = input
        .lines()
        .map(|line| -> u32 {
            let (range1, range2) = line.split_once(",").unwrap();

            let (range1_low, range1_high) = range1
                .split_once("-")
                .into_iter()
                .map(|s| (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap()))
                .next()
                .unwrap();
            let (range2_low, range2_high) = range2
                .split_once("-")
                .into_iter()
                .map(|s| (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap()))
                .next()
                .unwrap();

            match range1_low >= range2_low && range1_high <= range2_high {
                true => 1,
                false => match range2_low >= range1_low && range2_high <= range1_high {
                    true => 1,
                    false => 0,
                },
            }
        })
        .sum::<u32>();

    println!("{sum}");
}

fn part_two() {
    let input = get_input(4);
    let sum = input
        .lines()
        .map(|line| -> u32 {
            let (range1, range2) = line.split_once(",").unwrap();

            let (range1_low, range1_high) = range1
                .split_once("-")
                .into_iter()
                .map(|s| (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap()))
                .next()
                .unwrap();
            let (range2_low, range2_high) = range2
                .split_once("-")
                .into_iter()
                .map(|s| (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap()))
                .next()
                .unwrap();

            (range1_low..=range1_high)
                .find_map(|i| match (range2_low..=range2_high).contains(&i) {
                    true => Some(1),
                    false => None,
                })
                .unwrap_or(0)
        })
        .sum::<u32>();

    println!("{sum}");
}
