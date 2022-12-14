use std::{env, fs};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn get_input(day: usize) -> String {
    let path = env::current_dir().unwrap();
    let input_path = format!("src/days/day{}/input",day);
    fs::read_to_string(path.join(input_path)).unwrap()
}
