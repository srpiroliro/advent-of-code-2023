use std::time::{Instant, Duration};


fn get_free(matrix:&Vec<Vec<char>>) -> (Vec<usize>,Vec<usize>) {
    let cols: usize = matrix[0].len();
    let rows = matrix.len();

    let fc = (0..cols).filter_map(|col| {
        if (0..rows).all(|row| matrix[row][col] == '.') {
            Some(col)
        } else {
            None
        }
    }).collect();
    
    let fr = matrix.iter().enumerate().filter_map(|(i, row)| {
        if row.iter().all(|&c| c == '.') {
            Some(i)
        } else {
            None
        }
    })
    .collect();

    (fr,fc)
}

fn get_galaxies(matrix:&Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxy_points:Vec<Point> = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxy_points.push(Point{x, y});
            }
        }
    }

    galaxy_points
}

fn parse_map(input:&str) -> Vec<Vec<char>> {
    input.trim().lines().map(|line| line.trim().chars().collect()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize
}

fn count_values_in_range(l: &Vec<usize>, a: usize, b: usize) -> usize {
    l.iter()
        .filter(|&&row| row >= a && row <= b)
        .count()
}

pub fn solution(input:&str) -> (usize, Duration) {
    let start = Instant::now();

    let mulitplier = 1_000_000;
    
    let raw_image: Vec<Vec<char>> = parse_map(input);
    
    let galaxies = get_galaxies(&raw_image);
    let (free_rows,free_cols) = get_free(&raw_image);
    
    let mut result = 0;
    for (i, p1) in galaxies.iter().enumerate() {
        for p2 in galaxies.iter().skip(i+1) {
            let (r, c) = (p1.y, p1.x);
            let (r2, c2) = (p2.y, p2.x);

            let mut distance = c.abs_diff(c2) + r.abs_diff(r2);
            distance += count_values_in_range(&free_cols, c.min(c2), c.max(c2)) * (mulitplier - 1);
            distance += count_values_in_range(&free_rows, r.min(r2), r.max(r2)) * (mulitplier - 1);
            result += distance;
        }
    }

    (result, start.elapsed())
}