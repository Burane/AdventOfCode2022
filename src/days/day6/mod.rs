use super::get_input;

/**²
 * --- Day 6: ---
 *
 * https://adventofcode.com/2022/day/6
 */
pub fn solve() {
    println!("Day 6 :");
    part_one();
    part_two();
}

fn part_one() {
    let input = get_input(6);
    let chars: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let mut vec = [
            chars.get(i).unwrap(),
            chars.get(i + 1).unwrap(),
            chars.get(i + 2).unwrap(),
            chars.get(i + 3).unwrap(),
        ]
        .to_vec();
        let vec_len = vec.len();
        vec.sort();
        vec.dedup();
        let vec_len_dedup = vec.len();
        if vec_len == vec_len_dedup {
            println!("{:?}", vec);
            break;
        }
        i += 1;
    }
    i += 4;
    println!("{i}");
}

fn part_two() {
    let input = get_input(6);
    let chars: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let mut vec: Vec<char> = Vec::new();
        for j in 0..14 {
            if chars.len() > i + j {
                vec.push(chars.get(i + j).unwrap().to_owned())
            }
        }
        let vec_len = vec.len();
        vec.sort();
        vec.dedup();
        let vec_len_dedup = vec.len();
        if vec_len == vec_len_dedup {
            println!("{:?}", vec);
            break;
        }
        i += 1;
    }
    i += 14;
    println!("{i}");
}
