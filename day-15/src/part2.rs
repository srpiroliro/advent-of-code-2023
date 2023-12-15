
#[derive(Debug,Clone,Eq)]
struct Lens {
    name: String,
    focal_length: usize,
}

impl Lens {
    fn new(name: &str, focal_length: usize) -> Self {
        Self {
            name: name.to_string(),
            focal_length,
        }
    }
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn hashit(piece: &str) -> usize {
    piece.as_bytes().iter().fold(0, |acc, &c| (acc + c as usize) * 17 % 256)
}

fn solution(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for instructions in input.split(",") {
        let dirty = instructions.replace("-","=");
        let name = dirty.split("=").next().unwrap();
        let box_id = hashit(name);


        match instructions.find("-") {
            Some(_) => {
                if let Some(pos) = boxes[box_id].iter().position(|x| x.name == name) {
                    boxes[box_id].remove(pos);
                }
            },
            None => {
                let box_id = hashit(name);
                let focal_length = instructions.split("=").last().unwrap().parse::<usize>().unwrap();

                if let Some(pos) = boxes[box_id].iter().position(|x| x.name == name) {
                    boxes[box_id][pos].focal_length = focal_length;
                } else {
                    boxes[box_id].push(Lens::new(name, focal_length));
                }
            }
        }
        
    }

    (0..boxes.len())
        .map(|bi| 
            (0..boxes[bi].len())
                .map(|li| (bi+1) * (li+1) * boxes[bi][li].focal_length)
                .sum::<usize>())
        .sum()
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