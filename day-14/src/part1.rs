fn move_zeros(segment: &str) -> String {
    let zeros = segment.chars().filter(|&c| c == 'O').count();
    let others = segment.chars().filter(|&c| c != 'O').collect::<String>();
    format!("{}{}", "O".repeat(zeros), others)
}

fn tilt(segment: &str) -> String {
    segment.split("#").map(move_zeros).collect::<Vec<String>>().join("#")
}

fn get_cols(platform: &Vec<Vec<char>>) -> Vec<String> {
    (0..platform[0].len())
        .map(|i| platform.iter().map(|row| row[i]).collect())
        .collect()
}

fn create_platform(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn solution(input: &str) -> usize {
    let platform = create_platform(input);

    get_cols(&platform).iter().map(|col: &String| {      
        tilt(col).chars().rev().enumerate().map(|(i, c)| {
            if c == 'O' { i+1 } else { 0 }
        }).sum::<usize>()
    }).sum()
}

pub fn execute() {
    println!("[ -- PART 1 -- ]");

    let input = crate::aoc::read_input(None);
    let test_input = "O....#....
                            O.OO#....#
                            .....##...
                            OO.#O....O
                            .O.....O#.
                            O.#..O.#.#
                            ..O..#O..O
                            .......O..
                            #....###..
                            #OO..#....";

    let (result, duration) = crate::aoc::timeit(solution, test_input);
    println!("Test result: {:?} ( {:?} )", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ( {:?} )", result, duration);

    println!();
}
