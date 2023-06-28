use std::fs;

use day3;

#[test]
fn part1() {
    let f = fs::read_to_string("tests/data/input.txt").expect("correct input file");
    assert_eq!(day3::solve_part1(f.lines()), 7872);
}

//#[test]
//fn part2() {
//    let f = fs::read_to_string("tests/data/input.txt").expect("correct input file");
//    assert_eq!(day3::solve_part2(f.lines()), 12526);
//}
