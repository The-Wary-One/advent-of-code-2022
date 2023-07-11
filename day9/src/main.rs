use std::fs;

fn main() {
    let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    println!("day9 part1 result = {}", day9::solve_part1(f.lines()));
    //println!("day9 part2 result = {}", day9::solve_part2(f.lines()));
}
