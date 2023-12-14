struct Landscape {
    grid: Vec<Vec<char>>,
}

impl Landscape {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn get_symetrical(&self) -> usize {
        self.is_symmetrical(&self.rows()) * 100 + self.is_symmetrical(&self.cols())
    }

    fn rows(&self) -> Vec<String> {
        self.grid.iter().map(|row| row.iter().collect()).collect()
    }

    fn cols(&self) -> Vec<String> {
        (0..self.grid[0].len())
            .map(|i| (0..self.grid.len()).map(|j| self.grid[j][i]).collect())
            .collect()
    }

    // returns the amount of characters which are not symetrical. lower the better.
    fn is_symmetrical(&self, data: &Vec<String>) -> usize {
        for r in 1..data.len() {
            let above = &data[..r];
            let below = &data[r..];

            if self.differences(above, below) == 1 {
                return r;
            }
        }

        0
    }

    fn differences(&self, above: &[String], below: &[String]) -> usize {
        above
            .iter()
            .rev()
            .zip(below.iter())
            .flat_map(|(a, b)| a.chars().zip(b.chars()))
            .filter(|&(c, d)| c != d)
            .count()
    }
}

fn build_grids(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .trim()
        .split("\n\n")
        .map(|drawing| {
            drawing
                .trim()
                .lines()
                .map(|line| line.trim().chars().collect())
                .collect()
        })
        .collect()
}

fn solution(input: &str) -> usize {
    build_grids(input)
        .iter()
        .map(|grid| Landscape::new(grid.to_vec()).get_symetrical())
        .sum()
}

pub fn execute() {
    println!("[ -- PART 2 -- ]");

    let input = crate::aoc::read_input(None);
    let test_input = "#.##..##.
                            ..#.##.#.
                            ##......#
                            ##......#
                            ..#.##.#.
                            ..##..##.
                            #.#.##.#.

                            #...##..#
                            #....#..#
                            ..##..###
                            #####.##.
                            #####.##.
                            ..##..###
                            #....#..#";

    let (result, duration) = crate::aoc::timeit(solution, test_input);
    println!("Test result: {:?} ( {:?} )", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ( {:?} )", result, duration);

    println!();
}
