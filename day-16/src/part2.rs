use std::collections::HashSet;
use std::cmp::max;

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Step {
    direction: usize,
    position: (isize, isize),
}

impl Step {
    fn new(direction: usize, position: (isize, isize)) -> Step {
        Step {
            direction,
            position,
        }
    }
}

fn traverse(
    current_pos: (isize, isize),
    coming_from: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    directions_map: &mut HashSet<Step>,
) -> usize {
    // Check if the current position is out of bounds or already visited
    if current_pos.0 < 0
        || current_pos.1 < 0
        || current_pos.0 >= grid.len() as isize
        || current_pos.1 >= grid[0].len() as isize
        || !directions_map.insert(Step::new(coming_from, current_pos))
    {
        return 0;
    }

    let (x, y) = (current_pos.0 as usize, current_pos.1 as usize);

    let mut result = 0;
    if !visited[x][y] {
        visited[x][y] = true;
        result+=1
    }

    let next_pos = |dy: isize, dx: isize| (current_pos.0 + dy, current_pos.1 + dx);
    match grid[x][y] {
        '|' => {
            if coming_from != DOWN {
                result += traverse(next_pos(1, 0), UP, grid, visited, directions_map);
            }
            if coming_from != UP {
                result += traverse(next_pos(-1, 0), DOWN, grid, visited, directions_map);
            }
        }
        '-' => {
            if coming_from != RIGHT {
                result += traverse(next_pos(0, 1), LEFT, grid, visited, directions_map);
            }
            if coming_from != LEFT {
                result += traverse(next_pos(0, -1), RIGHT, grid, visited, directions_map);
            }
        }
        '.' => {
            match coming_from {
                UP => result += traverse(next_pos(1, 0), UP, grid, visited, directions_map),
                DOWN => result += traverse(next_pos(-1, 0), DOWN, grid, visited, directions_map),
                LEFT => result += traverse(next_pos(0, 1), LEFT, grid, visited, directions_map),
                RIGHT => result += traverse(next_pos(0, -1), RIGHT, grid, visited, directions_map),
                _ => {}
            }
        }
        '\\' => {
            // Change direction diagonally
            match coming_from {
                UP => result += traverse(next_pos(0, 1), LEFT, grid, visited, directions_map),
                DOWN => result += traverse(next_pos(0, -1), RIGHT, grid, visited, directions_map),
                LEFT => result += traverse(next_pos(1, 0), UP, grid, visited, directions_map),
                RIGHT => result += traverse(next_pos(-1, 0), DOWN, grid, visited, directions_map),
                _ => {}
            }
        }
        _ => { // /
            match coming_from {
                UP => result += traverse(next_pos(0, -1), RIGHT, grid, visited, directions_map),
                DOWN => result += traverse(next_pos(0, 1), LEFT, grid, visited, directions_map),
                LEFT => result += traverse(next_pos(-1, 0), DOWN, grid, visited, directions_map),
                RIGHT => result += traverse(next_pos(1, 0), UP, grid, visited, directions_map),
                _ => {}
            }
        }
    }

    result
}

fn journey(
    current_pos: (isize, isize),
    coming_from: usize,
    grid: &Vec<Vec<char>>
) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut directions_map = HashSet::new();

    traverse(current_pos, coming_from, grid, &mut visited, &mut directions_map)
}

fn build(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn solution(input: &str) -> usize {
    let grid = build(input);

    let mut max_value = 0;
    
    for y in 0..grid.len() {
        max_value = max(max_value, journey((y as isize, 0), LEFT, &grid));
        max_value = max(max_value, journey((y as isize, (grid.len()-1) as isize), RIGHT, &grid));
    }
    for x in 0..grid.len() {
        max_value = max(max_value, journey((0, x as isize), UP, &grid));
        max_value = max(max_value, journey(((grid.len()-1) as isize, x as isize), DOWN, &grid));
    }
    
    max_value
}

pub fn execute() {
    println!("[ -- PART 2 -- ]");

    let input = crate::aoc::read_input(None);
    let test_input = ".|...\\....
                            |.-.\\.....
                            .....|-...
                            ........|.
                            ..........
                            .........\\
                            ..../.\\\\..
                            .-.-/..|..
                            .|....-|.\\
                            ..//.|....";

    let (result, duration) = crate::aoc::timeit(solution, test_input);
    println!("Test result: {:?} ( {:?} )", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ( {:?} )", result, duration);

    println!();
}
