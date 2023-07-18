use std::fs;
use std::io::BufReader;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    env_logger::init();

    //let f = fs::read_to_string("./src/input.txt").expect("correct input file");
    //println!("day9 part1 result = {}", day9::solve_part1(f.lines()));
    //println!("day9 part2 result = {}", day9::solve_part2(f.lines()));

    let f = fs::File::open("./src/input.txt").expect("correct input file");
    let reader = BufReader::new(f);
    println!(
        "day9 part1 complex result = {}",
        day9::solve_part1_complex(reader)
    );
}
