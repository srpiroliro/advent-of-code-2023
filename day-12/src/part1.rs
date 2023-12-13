const OPERATIONAL:char = '.';
const DAMAGED:char = '#';
const UNKNOWN:char = '?';

fn arrangements(cfg:&str, nums:&Vec<usize>) -> usize {
    if cfg.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if cfg.contains('#') { 0 } else { 1 };
    }

    let mut result = 0;

    if cfg.starts_with(OPERATIONAL) || cfg.starts_with(UNKNOWN) {
        result += arrangements(&cfg[1..], nums);
    }

    if cfg.starts_with(DAMAGED) || cfg.starts_with(UNKNOWN) {
        // 1. check theres space to add the first number
        // 2. first characters should be all DAMAGED
        // 3. if cfg is empty, 0 will be returned closing down the recursion
        // 4. if after our group, there is a DAMAGED, 0 will be returned closing down the recursion (2 groups must be seperated by OPERATIONAL)
        
        if nums[0] <= cfg.len() && !cfg[0..nums[0]].contains(OPERATIONAL) && (nums[0] == cfg.len() || cfg.chars().nth(nums[0]).unwrap() != DAMAGED ){
            let starting_point = if nums[0] == cfg.len() {""} else {&cfg[nums[0]+1..]};
            result += arrangements(starting_point, &nums[1..].to_vec());
        }
    }

    result
}

fn solution(input:&str) -> usize {
    let total_arrangements = input.trim().lines().map(|line| {
        let parts:Vec<&str> =  line.trim().split_whitespace().collect();
        
        let row = parts[0];
        let contigous_groups:Vec<usize> = parts[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        
        arrangements(&row, &contigous_groups)
    }).sum();

    total_arrangements
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
    println!("Test result: {:?} ( {:?} )", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ( {:?} )", result, duration);
}