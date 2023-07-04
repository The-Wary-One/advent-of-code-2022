use std::collections::VecDeque;

fn find_marker_pos(datastream: &str, marker_len: usize) -> usize {
    let mut queue = VecDeque::with_capacity(4);
    for (pos, c) in datastream.char_indices() {
        // Assume there is always is a marker.
        if queue.len() == marker_len {
            // It means we found a marker in the previous loop.
            // The result is previous loop char pos + 1 OR this char pos.
            return pos;
        }
        // Did not find a marker previously.

        // Check if it already exists in the marker sequence.
        if queue.contains(&c) {
            // Pop the front chars until the duplicate char.
            loop {
                let front_char = queue.pop_front().expect("safe");
                if front_char == c {
                    break;
                }
            }
        }
        // Push the char to the end of the queue.
        queue.push_back(c);
    }
    panic!("impossible")
}

pub fn solve_part1(datastream: &str) -> usize {
    find_marker_pos(&datastream, 4)
}

pub fn solve_part2(datastream: &str) -> usize {
    find_marker_pos(&datastream, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
