use std::collections::HashMap;


const OPERATIONAL:char = '.';
const DAMAGED:char = '#';
const UNKNOWN:char = '?';

struct SpringRow {
    springs: String,
    groups: Vec<usize>,
}

impl SpringRow {
    fn new(line:String) -> Self {
        let parts:Vec<String> =  line.trim().split_whitespace().map(|a| a.to_string()).collect();

        SpringRow {
            springs: vec![parts[0].clone(); 5]
                        .join(UNKNOWN.to_string().as_str()),

            groups: vec![parts[1].clone(); 5]
                        .join(",")
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect(),
        }
    }

    fn arrangements(&self) -> usize {
        fn count(target:&str, nums:&Vec<usize>, cache:&mut HashMap<(String, Vec<usize>), usize>) -> usize {
            let key = (target.to_string(), nums.clone());
            if let Some(&cached_result) = cache.get(&key) {
                return cached_result;
            }

            if target.is_empty() {
                return if nums.is_empty() { 1 } else { 0 };
            }

            if nums.is_empty() {
                return if target.contains('#') { 0 } else { 1 };
            }

            let mut result = 0;

            if target.starts_with(OPERATIONAL) || target.starts_with(UNKNOWN) {
                result += count(&target[1..], nums, cache);
            }
        
            if target.starts_with(DAMAGED) || target.starts_with(UNKNOWN) {
                // 1. check theres space to add the first number
                // 2. first characters should be all DAMAGED
                // 3. if target is empty, 0 will be returned closing down the recursion
                // 4. if after our group, there is a DAMAGED, 0 will be returned closing down the recursion (2 groups must be seperated by OPERATIONAL)
                
                if nums[0] <= target.len() && !target[0..nums[0]].contains(OPERATIONAL) && (nums[0] == target.len() || target.chars().nth(nums[0]).unwrap() != DAMAGED ){
                    let starting_point = 
                        if nums[0] == target.len() {""} 
                        else {&target[nums[0]+1..]};
                    result += count(starting_point, &nums[1..].to_vec(), cache);
                }
            }

            cache.insert(key, result);
            result
        }

        count(self.springs.as_str(), self.groups.as_ref(), &mut HashMap::new())
    }
}



fn solution(input:&str) -> usize {
    let total_arrengements = input.trim().lines().map(|line| {
        let row = SpringRow::new(line.to_string());
        row.arrangements()

    }).sum();

    total_arrengements
}

pub fn execute() {
    println!("[ -- PART 2 -- ]");

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