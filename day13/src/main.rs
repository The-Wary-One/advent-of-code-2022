use std::fs;

fn main() {
    let i = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day13 part1 result = {}", day13::solve_part1(i.as_str()));
    println!("day13 part2 result = {}", day13::solve_part2(i.as_str()));
}
