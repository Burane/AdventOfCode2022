use super::get_input;

/**
 * --- Day 1: Calorie Counting ---
 *
 * https://adventofcode.com/2022/day/1
 */
pub fn solve() {
    let input = get_input(1);
    let soluce = input
        .split("\r\n\r\n")
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
