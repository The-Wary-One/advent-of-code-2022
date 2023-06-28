use std::fs;

use day2;

#[test]
fn part1() {
    let f = fs::read_to_string("tests/data/input.txt").expect("correct input file");
    assert_eq!(day2::solve_part1(f.lines()), 10994);
}

#[test]
fn part2() {
    let f = fs::read_to_string("tests/data/input.txt").expect("correct input file");
    assert_eq!(day2::solve_part2(f.lines()), 12526);
}
