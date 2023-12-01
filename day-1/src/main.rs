use std::fs;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const MIN_LEN:usize = 3;
const MAX_LEN:usize = 5;

fn main() {
    part1(); // ugly code

    part2(); // improved a bit
}

fn part1() {
    let contents = read_file("data.txt").unwrap();

    let mut numbers: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let all_numbers = line.chars().filter(|c| c.is_digit(10)).collect::<String>();

        if all_numbers.len() == 0 {
            continue;
        }

        let mut result = String::new();
        result.push(all_numbers.chars().nth(0).unwrap());
        result.push(all_numbers.chars().last().unwrap());

        if result.len() > 0 {
            numbers.push(result.parse::<i32>().unwrap());
        }
    }

    let sum = numbers.iter().sum::<i32>();

    println!("part 1: {}", sum);
}

fn part2() {
    let contents = read_file("data.txt").unwrap();

    let mut numbers: Vec<u32> = Vec::new();

    for mut line in contents.lines() {
        line = line.trim();

        let first:u32 = get_number_from_line(line, false);
        let last:u32 = get_number_from_line(line, true);

        numbers.push(first*10+last);
    }

    let sum = numbers.iter().sum::<u32>();

    println!("part 2: {}", sum);
}

fn get_number_from_line(line:&str, reverse:bool) -> u32 {
    for position in 0..line.len() {
        for i in 0..MAX_LEN {

            let num = get_num(line, position, i, reverse);
            if num != 10 {
                return num;
            }
        }
    }

    panic!("PANIC!!!: No number found. {}", line);
}

fn get_num(line:&str, position:usize, iteration_num:usize, reverse:bool) -> u32 {

    let mut start = position as isize;
    let mut end = (position + iteration_num) as isize;

    if reverse {
        end = line.len() as isize -1 - position as isize;
        start = end - iteration_num as isize;
    }

    if end >= line.len() as isize || start < 0 {
        return 10;
    }

    let new_char = &line.chars().nth(end as usize).unwrap();
    if new_char.is_digit(10) && iteration_num == 0 {
        return new_char.to_digit(10).unwrap()
    }

    let word = &line[start as usize ..(end+1) as usize];
    // println!("{} -> {} -> {}", line, word, reverse);
    
    if word.len() < MIN_LEN || !NUMBERS.contains(&word){
        return 10;
    }

    return NUMBERS.iter()
                .position(|&r| r == word)
                .unwrap() as u32 + 1;

}

fn read_file(file: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}