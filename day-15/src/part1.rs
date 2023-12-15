fn hashit(piece: &str) -> usize {
    piece.as_bytes().iter().fold(0, |acc, &c| (acc + c as usize) * 17 % 256)
}

fn solution(input: &str) -> usize {
    input.split(",").map(|piece| hashit(piece)).sum()
}

pub fn execute() {
    println!("[ -- PART 1 -- ]");

    let input = crate::aoc::read_input(None);
    let test_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let (result, duration) = crate::aoc::timeit(solution, test_input);
    println!("Test result: {:?} ( {:?} )", result, duration);

    let (result, duration) = crate::aoc::timeit(solution, input.as_str());
    println!("Result: {:?} ( {:?} )", result, duration);

    println!();
}
