use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn openings(c: char) -> Vec<Point> {
    match c {
        '|' => vec![Point::new(0, -1), Point::new(0, 1)],
        '-' => vec![Point::new(-1, 0), Point::new(1, 0)],
        'L' => vec![Point::new(0, -1), Point::new(1, 0)],
        'J' => vec![Point::new(0, -1), Point::new(-1, 0)],
        '7' => vec![Point::new(0, 1), Point::new(-1, 0)],
        _ => vec![Point::new(0, 1), Point::new(1, 0)],
    }
}

fn find_loop(
    map: &HashMap<Point, char>,
    prev: Point,
    cur: Point,
    distance: usize,
    loop_points: &mut HashSet<Point>,
) -> usize {
    if map[&cur] == 'S' {
        let distance = distance + 1;
        match distance % 2 {
            0 => return distance / 2,
            _ => return distance / 2 + 1,
        }
    }

    let nexts = openings(map[&cur]);
    let next = nexts.iter().find(|&&p| p != prev - cur).unwrap();
    loop_points.insert(cur + *next);
    find_loop(map, cur, cur + *next, distance + 1, loop_points)
}


fn in_loop(map: &HashMap<Point, char>, loop_points: &HashSet<Point>) -> HashSet<Point> {
    let mut inside = HashSet::new();

    let max_x = map.keys().map(|p| p.x).max().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();

    for x in 0..=max_x {
        let mut left = 0;
        let mut right = 0;

        for y in 0..=max_y {
            match loop_points.contains(&Point::new(x, y)) {
                true => match map[&Point::new(x, y)] {
                    '-' => {
                        left += 1;
                        right += 1
                    }
                    'L' => right += 1,
                    'F' => right += 1,
                    'J' => left += 1,
                    '7' => left += 1,
                    _ => (),
                },

                false => {
                    if left.min(right) % 2 == 1 {
                        inside.insert(Point::new(x, y));
                    }
                }
            }
        }
    }
    inside
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<Point, char>>();


    let start = map.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let next = *start + Point::new(0, 1);

    let mut loop_points = HashSet::from([*start, next]);
    
    let p1 = find_loop(&map, *start, next, 0, &mut loop_points);
    println!("p1: {}", p1);

    let inside = in_loop(&map, &loop_points);
    println!("p2: {}", inside.len());

}