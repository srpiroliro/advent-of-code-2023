use std::cmp::max;
use std::fs;

fn main() {
    let test_input = "Time:      7  15   30
Distance:  9  40  200";

    let input = read_file().unwrap();

    part2(&input);
}

fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    Ok(contents)
}


fn part1(input: &str) {
    let parts:Vec<&str>=input.lines().collect();
    
    let time_vec = parts[0].split(":").map(|p| p.trim()).collect::<Vec<&str>>()[1].split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distance_vec = parts[1].split(":").map(|p| p.trim()).collect::<Vec<&str>>()[1].split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();

    println!("time_vec: {:?}", time_vec);
    println!("distance_vec: {:?}", distance_vec);

    let mut winable_cnt: i32 = 1;

    for i in 0..time_vec.len() {
        let time = time_vec[i];
        let record = distance_vec[i];

        let mut cnt = 0;

        for i in 0+1..time {
            let distance = i * (time-i);
            
            if distance > record {
                cnt += 1;
            }
        }

        winable_cnt *= max(cnt, 1);
    }

    println!("part1: {}", winable_cnt);
}

fn part2(input: &str) {
    let parts:Vec<&str>=input.lines().collect();
    
    let time = parts[0].split(":").map(|p| p.trim()).collect::<Vec<&str>>()[1].replace(" ", "").parse::<u64>().unwrap();
    let distance = parts[1].split(":").map(|p| p.trim()).collect::<Vec<&str>>()[1].replace(" ", "").parse::<u64>().unwrap();

    println!("time_vec: {}", time);
    println!("distance_vec: {}", distance);

    let mut winable_cnt: i32 = 1;
    let mut cnt = 0;

    for i in 0+1..time {
        let d = i * (time-i);
        
        if d > distance {
            cnt += 1;
        }
    }

    winable_cnt *= max(cnt, 1);

    println!("part2: {}", winable_cnt);
}