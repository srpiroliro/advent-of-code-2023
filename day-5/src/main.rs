use std::{fs, collections::HashMap};
use std::cmp::{min, max};
use rayon::prelude::*;


fn main() {
    let input = read_file().unwrap();
    let test_input = "seeds: 79 14 55 13

                            seed-to-soil map:
                            50 98 2
                            52 50 48

                            soil-to-fertilizer map:
                            0 15 37
                            37 52 2
                            39 0 15

                            fertilizer-to-water map:
                            49 53 8
                            0 11 42
                            42 0 7
                            57 7 4

                            water-to-light map:
                            88 18 7
                            18 25 70

                            light-to-temperature map:
                            45 77 23
                            81 45 19
                            68 64 13

                            temperature-to-humidity map:
                            0 69 1
                            1 0 69

                            humidity-to-location map:
                            60 56 37
                            56 93 4";

    part2(&input);
}

fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    Ok(contents)
}


struct MapData {
    destination: usize,
    source: usize,
    range_length: usize
}

fn part1(input:&str) {
    let map_order = vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];
    let parts: Vec<&str> = input.split("\n\n").map(|p| p.trim()).collect();

    let mut values: Vec<usize> = parts[0].split(": ").collect::<Vec<&str>>()[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

    let maps: HashMap<String, Vec<MapData>> = generate_maps(parts);

    for map_name in map_order.iter() {
        let map_data = maps.get(&map_name.to_string()).unwrap();

        for value in &mut values {
            for map in map_data.iter() {
                
                if *value >= map.source && *value < map.source + map.range_length {
                    
                    let dif = *value - map.source;
                    *value = map.destination + dif;

                    break;
                }
            }
        }
    }

    values.sort();
    println!("Part 1: {}", values[0]);
}

fn generate_maps(parts:Vec<&str>) -> HashMap<String, Vec<MapData>> {
    let mut maps: HashMap<String, Vec<MapData>> = HashMap::new();

    for part in parts[1..].iter() {
        let lines: Vec<&str> = part.split("\n").map(|l| l.trim()).collect();
        let map_name = lines[0].split(": ").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[0];

        let mut map_data_vec: Vec<MapData> = Vec::new();
        for line in lines[1..].iter() {
            let line_data: Vec<usize> = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
            
            map_data_vec.push(MapData {
                destination: line_data[0],
                source: line_data[1],
                range_length: line_data[2]
            });
        }

        maps.insert(map_name.to_string(), map_data_vec);
    }

    maps
}


fn part2(input:&str) {
    let map_order = vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];
    let parts: Vec<&str> = input.split("\n\n").map(|p| p.trim()).collect();

    let seed_ranges: Vec<usize> = parts[0].split(": ").collect::<Vec<&str>>()[1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();


    let mut values: Vec<usize> = Vec::new();
    for i in (0..seed_ranges.len()).step_by(2) {
        let seed = seed_ranges[i];
        let range = seed_ranges[i+1];

        for j in 0..range {
            values.push(seed+j);
        }
    }


    let maps: HashMap<String, Vec<MapData>> = generate_maps(parts);

    values.par_iter_mut().for_each(|value| {
        for map_name in &map_order {
            let map_data = maps.get(&map_name.to_string()).unwrap();
            for map in map_data {
                if *value >= map.source && *value < map.source + map.range_length {
                    let dif = *value - map.source;
                    *value = map.destination + dif;
                    break;
                }
            }
        }
    });

    values.sort_unstable();
    println!("Part 2: {}", values[0]);
}