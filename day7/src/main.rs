use std::fs;

use day7;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day7 part1 result = {}", day7::solve_part1(&mut f.lines()));
    println!("day7 part2 result = {}", day7::solve_part2(&mut f.lines()));
}
