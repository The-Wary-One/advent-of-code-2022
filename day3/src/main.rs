use std::fs;

use day3;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day3 part1 result = {}", day3::solve_part1(f.lines()));
    println!("day3 part2 result = {}", day3::solve_part2(f.lines()));
}
