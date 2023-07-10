use std::fs;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day7 part1 result = {}", day8::solve_part1(f.lines()));
    //println!("day7 part2 result = {}", day8::solve_part2(f.lines()));
}
