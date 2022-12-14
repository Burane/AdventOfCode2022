use super::get_input;

/**
 * --- Day 1: Calorie Counting ---
 *
 * https://adventofcode.com/2022/day/1
 */
pub fn solve() {
    println!("Day 1 :");
    part_one();
    part_two();
}

fn part_one() {
    let input = get_input(1);
    let soluce = input
        .split("\n\n")
        .map(|split| {
            split
                .lines()
                .map(|s| s.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .max();

    println!(
        "The elf carrying the most calories is carrying a total of {} calories",
        soluce.unwrap()
    );
}

fn part_two() {
    let input = get_input(1);
    let mut sums: Vec<isize> = input
        .split("\n\n")
        .map(|split| {
            split
                .lines()
                .map(|s| s.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .collect();

    sums.sort();
    sums.reverse();
    let soluce = &sums.as_slice()[0..3].iter().sum::<isize>();

    println!(
        "The 3 elf carrying the most calories are carrying a total of {} calories",
        soluce
    );
}
