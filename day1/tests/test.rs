use std::fs;

use day1;

#[test]
fn part1() {
    let f = fs::read_to_string("tests/data/input.txt").expect("correct input file");
    assert_eq!(day1::solve_part1(f.lines()), 69177);
}

#[test]
fn part2() {
    let f = fs::read_to_string("tests/data/input.txt").expect("correct input file");
    assert_eq!(day1::solve_part2(f.lines()), 207456);
}
