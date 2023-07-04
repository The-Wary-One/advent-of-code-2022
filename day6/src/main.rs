use std::fs;

use day6;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day6 part1 result = {}", day6::solve_part1(f.as_str()));
    println!("day6 part2 result = {}", day6::solve_part2(f.as_str()));
}
