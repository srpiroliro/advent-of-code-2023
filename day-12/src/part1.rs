use std::time::Instant;
use rayon::prelude::*;

const OPERATIONAL:char = '.';
const DAMAGED:char = '#';
const UNKNOWN:char = '?';

// neccessary?
fn validate(row:&str, groups:&Vec<usize>) -> bool {
    let mut rex_str = r"^\.*".to_string();
    rex_str.push_str(&groups.iter().map(|n| format!("{}{}", r"#{", n)).collect::<Vec<String>>().join(&r"}\.+"));
    rex_str.push_str(&r"}\.*$");

    let rex = regex::Regex::new(rex_str.as_str()).unwrap();

    rex.is_match(row)
}

fn arrengements(row:&str, groups:&Vec<usize>) -> usize {
    let mut arrengements = 0;

    // list of ? positions in row.
    let unknowns:Vec<usize> = row.chars().enumerate().filter(|(_, c)| *c == UNKNOWN).map(|(i, _)| i).collect();

    let base = row.replace(UNKNOWN, OPERATIONAL.to_string().as_str());
    let max_num = usize::pow(2,unknowns.len() as u32);

    for i in max_num..(2*max_num) {
        let mut mutant = base.clone();
        for (j, bit) in format!("{:b}", i).chars().skip(1).enumerate() {
            if bit == '0' {
                continue;
            }
            let unknown_idx = unknowns[j];
            mutant.replace_range(unknown_idx..unknown_idx+1, DAMAGED.to_string().as_str());
        }

        let valid = validate(&mutant, &groups);

        if valid {
            arrengements += 1;
        }
    }


    arrengements
}

fn solution(input:&str) -> usize {
    let input: Vec<_> = input.trim().lines().collect();
    
    let total_arrengements:usize = input.par_iter().map(|line| {
        let parts:Vec<&str> =  line.trim().split_whitespace().collect();

        let row = parts[0];
        let contigous_groups:Vec<usize> = parts[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        let a = arrengements(&row, &contigous_groups);
        a
    }).sum();

    total_arrengements
}

pub fn execute() {
    println!("[ -- PART 1 -- ]");

    let input = crate::aoc::read_input(None);
    let test_input = "???.### 1,1,3
                            .??..??...?##. 1,1,3
                            ?#?#?#?#?#?#?#? 1,3,1,6
                            ????.#...#... 4,1,1
                            ????.######..#####. 1,6,5
                            ?###???????? 3,2,1";

    let (result, duration) = crate::aoc::timeit(solution, test_input);
    println!("Test result: {:?} ({:?})", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ({:?})", result, duration);
}