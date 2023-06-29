use std::fs;

use day1;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day1 part1 result = {}", day1::solve_part1(f.lines()));
    println!("day1 part2 result = {}", day1::solve_part2(f.lines()));
}
