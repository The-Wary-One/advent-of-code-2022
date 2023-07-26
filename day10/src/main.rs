use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let f = fs::File::open("./src/input.txt").expect("correct input file");
    let reader = BufReader::new(f);
    //println!(
    //    "day10 part1 result = {}",
    //    day10::solve_part1(reader.lines())
    //);
    println!(
        "day10 part2 result = \n{}",
        day10::solve_part2(reader.lines())
    );
}
