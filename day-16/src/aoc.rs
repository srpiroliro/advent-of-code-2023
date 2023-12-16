use std::fs;
use std::time::{Instant, Duration};


pub fn timeit<F,T>(f:F, input: &str) -> (T, Duration) 
where F: Fn(&str) -> T
{
    let start = Instant::now();
    let result = f(input);

    (result, start.elapsed())
}

pub fn read_input(path:Option<&str>) -> String {
    let filepath = path.unwrap_or("input.txt");
    fs::read_to_string(filepath)
        .expect("Could not read input file")
}
