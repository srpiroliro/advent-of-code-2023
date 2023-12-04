use std::{fs, collections::HashMap};

fn main() {
    let test_input = ".333.114..
                            ..*.......
                            .357..633.
                            ......#...
                            ..2*......
                            ....44.58.
                            ..592..*..
                            .......55.
                            ...$......
                            .664.598..";

    let input = read_file().unwrap();
    // part1(&input);

    part2(&input);
}


fn part1(input:&str){
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.trim().chars().collect()).collect();

    let mut result: isize = 0;

    let mut y = 0;
    while y < matrix.len() {
        let mut x = 0;
        
        let mut found_digit = false;
        let mut validator = false;

        let mut number = String::new();

        let line: &Vec<char> = &matrix[y];
        while x < line.len() {
            let current = line[x];
            let upper: char = if y > 0 { matrix[y-1][x] } else { ' ' };
            let lower: char = if y+1 < matrix.len() { matrix[y+1][x] } else { ' ' };

            let mut current_val = is_valid(&upper) || is_valid(&current) || is_valid(&lower);

            if current.is_digit(10) {
                number.push(current);
                current_val = current_val || validator;
                found_digit = true;
            } else {
                if found_digit {
                    if current_val || validator {
                        result += number.parse::<isize>().unwrap();
                    }                        
                    number = String::new();
                }
                
                found_digit = false;
            }
            
            validator = current_val;
            x += 1;

            // println!("");
        }

        if found_digit && validator {
            result += number.parse::<isize>().unwrap();
        }

        // println!("\nNEW ROW\n");
        y += 1;
    }

    println!("Part 1: {}", result);

}

fn is_valid(c:&char) -> bool {
    !c.is_digit(10) && *c != '.' && *c != ' '
}

//////////////////////////////////////////////////////

fn part2(input:&str){
    let matrix: Vec<Vec<char>> = input.lines().map(|x| x.trim().chars().collect()).collect();
    let mut result: isize = 0;

    for y in 0..matrix.len() {
        let line: &Vec<char> = &matrix[y];
        for x in 0..line.len() {
            let current = line[x];

            if current == '*' {
                let nearest_digits:Vec<isize>  = find_nearby_digits(&matrix, x, y);
                println!("{} {} :: {:?} -- {}", x, y, nearest_digits, nearest_digits.len() == 2);

                if nearest_digits.len() == 2 {
                    result += nearest_digits.iter().product::<isize>();
                }
            }
        }
    }

    println!("Part 2: {}", result);
}

fn find_nearby_digits(matrix:&Vec<Vec<char>>, x:usize, y:usize) -> Vec<isize> {
    let mut digits: Vec<isize> = Vec::new();
    
    for dy in -1..=1 {
        let mut visited: Vec<isize> = Vec::new();
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < matrix[0].len() as isize && ny >= 0 && ny < matrix.len() as isize {
                let current = matrix[ny as usize][nx as usize];
                if current.is_digit(10) {
                    let (start, full_digit) = get_full_digit(&matrix[ny as usize], nx as usize);

                    if visited.contains(&start) {
                        continue;
                    }
                    digits.push(full_digit);
                    visited.push(start);
                }
            }
        }
    }

    digits
}

fn get_full_digit(row:&Vec<char>, x:usize) -> (isize, isize) {
    let mut result:String = String::new();


    for i in x..row.len() {
        let current = row[i];

        if current.is_digit(10) {
            result+=&current.to_string();
        } else {
            break;
        }
    }

    let mut first = x as isize;
    for i in (0..x).rev() {
        let current = row[i];

        if current.is_digit(10) {
            result = current.to_string() + &result;
            first = i as isize;
        } else {
            break;
        }
    }

    (first, result.parse().unwrap())
}



fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    Ok(contents)
}