use std::time::{Instant, Duration};


fn get_free_rows(matrix:&Vec<Vec<char>>) -> Vec<usize> {
    matrix.iter().enumerate().filter_map(|(i, row)| {
        if row.iter().all(|&c| c == '.') {
            Some(i)
        } else {
            None
        }
    })
    .collect()
}

fn get_free_columns(matrix:&Vec<Vec<char>>) -> Vec<usize> {
    let cols: usize = matrix[0].len();
    let rows = matrix.len();

    (0..cols).filter_map(|col| {
        if (0..rows).all(|row| matrix[row][col] == '.') {
            Some(col)
        } else {
            None
        }
    }).collect()
}

fn build(matrix:&Vec<Vec<char>>, spacing: usize) -> (Vec<Point>, Vec<Vec<char>>) {
    let mut result = Vec::new();        

    let free_cols:Vec<usize> = get_free_columns(matrix);
    let free_rows:Vec<usize> = get_free_rows(matrix);

    let mut galaxy_points:Vec<Point> = Vec::new();

    let mut y=0;
    for (i, row) in matrix.iter().enumerate() {
        let mut x=0;
        let mut new_row = Vec::new();
        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxy_points.push(Point{x, y});
            }

            let iters = if free_cols.contains(&i) { spacing } else { 1 };
            for _ in 0..iters {
                new_row.push(c);
                x+=1;
            }
        }

        let iters = if free_rows.contains(&i) { spacing } else { 1 };
        for _ in 0..iters {
            result.push(new_row.clone());
            y+=1;
        }
    }  

    (galaxy_points, result)
}


fn parse_map(input:&str) -> Vec<Vec<char>> {
    input.trim().lines().map(|line| line.trim().chars().collect()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn distance(&self, other:&Point) -> u32 {
        let x = self.x as i32 - other.x as i32;
        let y = self.y as i32 - other.y as i32;

        (x.abs() + y.abs()) as u32
    }
}

#[derive(Debug, Eq, Clone, Copy, Hash)]
struct Pair {
    a: Point,
    b: Point
}

impl PartialEq for Pair {
    fn eq(&self, other:&Pair) -> bool {
        (self.a == other.a && self.b == other.b) || 
            (self.a == other.b && self.b == other.a)
    }
}


pub fn solution(input:&str) -> (usize, Duration) {
    let start = Instant::now();
    let mut result = 0;

    let raw_image: Vec<Vec<char>> = parse_map(input);
    let (galaxies, _) = build(&raw_image, 2);

    for (i,p1) in galaxies.iter().enumerate() {
        for p2 in galaxies.iter().skip(i+1) {
            result += p1.distance(p2) as usize;
        }
    }

    (result, start.elapsed())
}