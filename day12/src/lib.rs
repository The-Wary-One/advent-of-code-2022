use height_map::HeightMap;

mod graph;
mod heap;
mod height_map;

pub fn solve_part1(input: &str) -> usize {
    let hm: HeightMap = input.try_into().expect("safe");
    hm.find_shortest_path_part1()
}

pub fn solve_part2(input: &str) -> usize {
    let hm: HeightMap = input.try_into().expect("safe");
    hm.find_shortest_path_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 29);
    }
}
