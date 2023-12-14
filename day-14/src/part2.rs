use std::collections::{HashMap, HashSet};

fn to_str(platform: &Vec<Vec<char>>) -> String {
    platform
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn calc(platform: &Vec<Vec<char>>) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(y, row)| row.iter().filter(|&c| *c == 'O').count() * (y + 1))
        .sum()
}

fn move_zeros(segment: &str, invert: bool) -> String {
    let zeros = segment.chars().filter(|&c| c == 'O').count();
    let others = segment.chars().filter(|&c| c != 'O').collect::<String>();

    match invert {
        true => format!("{}{}", others, "O".repeat(zeros)),
        false => format!("{}{}", "O".repeat(zeros), others),
    }
}

fn tilt_zeros(segment: &String, invert: bool) -> String {
    segment
        .split("#")
        .map(|piece| move_zeros(piece, invert))
        .collect::<Vec<String>>()
        .join("#")
}

// invert used for south and east.
fn tilt(platform: &Vec<Vec<char>>, invert: bool) -> Vec<Vec<char>> {
    platform
        .iter()
        .map(|row| row.iter().collect())
        .map(|line| tilt_zeros(&line, invert))
        .map(|row| row.chars().collect())
        .collect()
}

// rotates 90 degrees (invert:true = -90 degrees)
// used for north and south.
fn rotate(matrix: &Vec<Vec<char>>, invert: bool) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rotated = vec![vec![' '; rows]; cols];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            let y = if invert { cols - 1 - j } else { j };
            let x = if invert { i } else { rows - 1 - i };

            rotated[y][x] = val;
        }
    }

    rotated
}

fn cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_platform = platform.clone();
    for _ in 0..4 {
        new_platform = rotate(&new_platform, false);
        new_platform = tilt(&new_platform, true);
    }

    new_platform
}

fn create_platform(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn solution(input: &str) -> usize {
    let original_platform = create_platform(input);

    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut all: Vec<Vec<Vec<char>>> = Vec::new();

    seen.insert(to_str(&original_platform), 0);
    all.push(original_platform.clone());

    let mut platform: Vec<Vec<char>> = original_platform.clone();
    for i in 1..1000000000 {
        platform = cycle(&platform);

        let sp = to_str(&platform);
        if seen.contains_key(&sp) {
            let first_seen = seen.get(&sp).unwrap().clone();
            let golden_idx = (1000000000 - first_seen) % (i - first_seen) + first_seen;

            return calc(&all[golden_idx]);
        }

        seen.insert(sp, i);
        all.push(platform.clone());
    }

    0
}

pub fn execute() {
    println!("[ -- PART 2 -- ]");

    let input = crate::aoc::read_input(None);
    let test_input = "OOOO.#.O..
                            OO..#....#
                            OO..O##..O
                            O..#.OO...
                            ........#.
                            ..#....#.#
                            ..O..#.O.O
                            ..O.......
                            #....###..
                            #....#....";

    let (result, duration) = crate::aoc::timeit(solution, test_input);
    println!("Test result: {:?} ( {:?} )", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ( {:?} )", result, duration);

    println!();
}
