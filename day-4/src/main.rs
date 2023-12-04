use std::collections::{HashMap, HashSet};
use std::fs;

use regex::Regex;

fn main() {
    let input = read_file().unwrap();
    let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    part2(&input);
}

fn part1(input: &str) {
    let clean_input = input.trim().replace("  ", " ");

    let games = clean_input
        .lines()
        .map(|l| l.trim().split(":").collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>();

    let mut result = 0;
    for game in games {
        let mut local_result = 0;
        let game_parts = game.split("|").collect::<Vec<&str>>();

        let golden: Vec<usize> = game_parts[0]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        for number in game_parts[1]
            .trim()
            .split_whitespace()
            .map(|n: &str| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        {
            if golden.contains(&number) {
                if local_result == 0 {
                    local_result = 1;
                } else {
                    local_result *= 2;
                }
            }
        }

        result += local_result;
    }

    println!("part 1: {}", result);
}

fn part2(input: &str) {
    let re = Regex::new(r"[ \t]+").unwrap();
    let clean_input = re.replace_all(input.trim(), " ");

    let lines = clean_input
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>();

    let mut history_map: HashMap<usize, usize> = HashMap::new();
    let total_games = lines.len();

    for i in 1..=total_games {
        history_map.insert(i, 1);
    }

    for line in lines {
        let parts = line.split(":").collect::<Vec<&str>>();
        let game_number = parts[0].trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        let game_parts = parts[1].trim().split("|").collect::<Vec<&str>>();

        let golden: HashSet<&str> = game_parts[0].trim().split_whitespace().collect();
        let tickets: HashSet<&str> = game_parts[1].trim().split_whitespace().collect();

        let winning_len = tickets.intersection(&golden).count();

        for i in game_number + 1..=std::cmp::min(game_number + winning_len, total_games) {
            let instances =  *history_map.get(&game_number).unwrap();
            let current = history_map.entry(i).or_insert(0);
            *current += instances;
        }
    }

    let result = history_map.values().sum::<usize>();
    println!("part 2: {}", result);
}


fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    Ok(contents)
}
