use std::fs;

use day5;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day5 part1 result = {}", day5::solve_part1(f.lines()));
    println!("day5 part2 result = {}", day5::solve_part2(f.lines()));
}
