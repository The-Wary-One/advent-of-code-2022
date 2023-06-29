use std::fs;

use day2;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day2 part1 result = {}", day2::solve_part1(f.lines()));
    println!("day2 part2 result = {}", day2::solve_part2(f.lines()));
}
