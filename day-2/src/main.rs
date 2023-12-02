use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = read_file("data.txt").unwrap();
    
    let result1 = part1();
    println!("part 1: {}", result1);

    let result2 = part2(contents);
    println!("part 2: {}", result2);
}


fn part1() -> u32 {
    let mut possible_cubes = HashMap::new();
    possible_cubes.insert("red", 12);
    possible_cubes.insert("green", 13);
    possible_cubes.insert("blue", 14);
    
    let contents = read_file("data.txt").unwrap();
    let mut games_cnt = 0;

    for line in contents.lines() {
        let parts = line.trim().split(":").collect::<Vec<&str>>();

        let game_number:u32 = get_number(parts[0]);
        let mut cubes_plays = parts[1].split(";").collect::<Vec<&str>>();

        let mut is_possible = true;
        while is_possible && !cubes_plays.is_empty() {
            let cubes_play = cubes_plays.pop().unwrap();

            let cubes = get_cubes(cubes_play);
            for (color, count) in cubes {
                let max_amount = possible_cubes.get(color.as_str()).unwrap();
                if count > *max_amount {
                    is_possible = false;
                    break;
                }
            }
        }

        if is_possible {
            games_cnt += game_number;
        }
    }

    games_cnt
}


fn part2(contents:String) -> u32 {
    let mut result_cnt = 0;

    for line in contents.lines() {
        let mut needed_cubes = HashMap::new();
        needed_cubes.insert("red".to_string(), 0);
        needed_cubes.insert("green".to_string(), 0);
        needed_cubes.insert("blue".to_string(), 0);

        let parts = line.trim().split(":").collect::<Vec<&str>>();

        let mut cubes_plays = parts[1].split(";").collect::<Vec<&str>>();

        while !cubes_plays.is_empty() {
            let cubes_play = cubes_plays.pop().unwrap();

            let cubes = get_cubes(cubes_play);
            for (color, count) in cubes {
                let needed_amount = needed_cubes.get(color.as_str()).unwrap();
                
                if count > *needed_amount {
                    needed_cubes.insert(color, count);
                }
            }
        }

        let red = needed_cubes.get("red").unwrap();
        let green = needed_cubes.get("green").unwrap();
        let blue = needed_cubes.get("blue").unwrap();
        
        result_cnt += red*green*blue;
    }

    result_cnt
}


fn get_number(game_part:&str) -> u32 {
    let game_number = game_part.split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
    
    game_number
}

fn get_cubes(cubes_part:&str) -> HashMap<String, u32> {
    let mut cubes_map:HashMap<String, u32> = HashMap::new();
    let cubes = cubes_part.split(",").collect::<Vec<&str>>();

    for cube in cubes {
        let clean_cube = cube.trim();

        let cube_parts = clean_cube.split(" ").collect::<Vec<&str>>();
        cubes_map.insert(cube_parts[1].to_string() ,cube_parts[0].trim().parse::<u32>().unwrap());
    }

    cubes_map
}

fn read_file(filename:&str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}