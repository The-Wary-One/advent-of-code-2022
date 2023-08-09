use std::fs;

fn main() {
    let i = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day12 part1 result = {}", day12::solve_part1(i.as_str()));
    println!("day12 part2 result = {}", day12::solve_part2(i.as_str()));
}
