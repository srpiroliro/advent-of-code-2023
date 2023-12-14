

struct Landscape {
    grid: Vec<Vec<char>>,
}

impl Landscape {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn get_symetrical(&self) -> usize {
        Landscape::is_symmetrical(&self.rows())*100 +
            Landscape::is_symmetrical(&self.cols()) 
    }

    fn rows(&self) -> Vec<String> {
        self.grid.iter().map(|row| row.iter().collect()).collect()
    }

    fn cols(&self) -> Vec<String> {
        (0..self.grid[0].len()).map(|i| 
            (0..self.grid.len()).map(|j| self.grid[j][i]).collect()).collect()
    }

    // returns the amount of characters which are not symetrical. lower the better.
    fn is_symmetrical(data:&Vec<String>) -> usize {
        for r in 1..data.len() {
            let above: &[String] = &data[..r];
            let below = &data[r..];

            let min_len = std::cmp::min(above.len(), below.len());
            let above_reversed = above.iter().rev().take(min_len);
            let below_sliced = below.iter().take(min_len);
            
            if above_reversed.eq(below_sliced) {
                return r;
            }
        }

        0
    
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

    let mut notes = 0;
    for grid in build_grids(input) {

        let landscape = Landscape::new(grid);
        notes+=landscape.get_symetrical();

    }

    notes
}

pub fn execute() {
    println!("[ -- PART 1 -- ]");

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
