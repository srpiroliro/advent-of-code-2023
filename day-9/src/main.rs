
use std::fs;
fn main() {
    let test_input = "0 3 6 9 12 15
                            1 3 6 10 15 21
                            10 13 16 21 30 45";
    let input = read_file();

    part1(&test_input);
    part1(&input);

    println!("------------------");

    part2(&test_input);
    part2(&input);
}

fn part1(input:&str) {
    let mut results = 0;

    for line in input.lines() {
        let numbers: Vec<isize> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut difs: Vec<Vec<isize>> = Vec::new();
        let mut end = false;
        let mut current_vec_id=0;
        let mut lasts:Vec<isize> = Vec::new();

        lasts.push(*numbers.iter().last().unwrap());
        difs.push(numbers.clone());

        while !end {
            let current_vec = difs.get(current_vec_id).unwrap();
            let mut current_difs: Vec<isize> = Vec::new();
            let mut last: isize = 0;

            end = true;
            for i in 1..current_vec.len() {
                let dif=current_vec[i]-current_vec[i-1];
                current_difs.push(dif);
                last = dif;

                if dif != 0 {
                    end = false;
                }
            }

            lasts.push(last);
            difs.push(current_difs);
            current_vec_id += 1;
        }

        let mut prev =0;
        let mut is_first = true;
        for number in lasts.iter_mut().rev() {
            if is_first {
                is_first = false;
                continue;
            }

            *number += prev;

            prev = *number;
        }

        results += lasts[0];
    }

    println!("Results: {}", results)
}

fn part2(input:&str) {
    let mut results = 0;

    for line in input.lines() {
        let numbers: Vec<isize> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut difs: Vec<Vec<isize>> = Vec::new();
        let mut end = false;
        let mut current_vec_id=0;
        let mut firsts:Vec<isize> = Vec::new();

        firsts.push(*numbers.iter().nth(0).unwrap());
        difs.push(numbers.clone());

        while !end {
            let current_vec = difs.get(current_vec_id).unwrap();
            let mut current_difs: Vec<isize> = Vec::new();
            let mut first: isize = 0;

            end = true;
            for i in 1..current_vec.len() {
                let dif=current_vec[i]-current_vec[i-1];
                current_difs.push(dif);
                
                if i==1 {
                    first = dif;
                }
                if dif != 0 {
                    end = false;
                }
            }

            firsts.push(first);
            difs.push(current_difs);
            current_vec_id += 1;

        }

        let mut prev =0;
        let mut is_first = true;
        for number in firsts.iter_mut().rev() {
            if is_first {
                is_first = false;
                continue;
            }

            *number -= prev;
            prev = *number;
        }

        results += firsts[0];
    }

    println!("Results: {}", results)
}

fn read_file() -> String {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    contents
}
