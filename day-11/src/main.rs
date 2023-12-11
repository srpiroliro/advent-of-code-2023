use std::{fs, time::Instant};

mod part1;
mod part2;


fn main() {

    let input = fs::read_to_string("input.txt")
                    .expect("404 file not found :(");

    
    let test_input = "...#......
                            .......#..
                            #.........
                            ..........
                            ......#...
                            .#........
                            .........#
                            ..........
                            .......#..
                            #...#.....";

    println!("[ -- PART 1 -- ]");

    let (result, duration) = part1::solution(&test_input);
    println!("\ttest: {} - {:?}", result, duration);

    let (result, duration) = part1::solution(&input);
    println!("\tpuzzle input: {} - {:?}", result, duration);

    
    println!();

    println!("[ -- PART 2 -- ]");
    
    let (result, duration) = part2::solution(&test_input);
    println!("\ttest: {} - {:?}", result, duration);
    
    let (result, duration) = part2::solution(&input);
    println!("\tpuzzle input: {} - {:?}", result, duration);
}
