use std::fs;

fn main() {
    let i = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day11 part1 result = {}", day11::solve_part1(i.as_str()));
    println!("day11 part2 result = {}", day11::solve_part2(i.as_str()));
}
