use std::{fs, collections::HashMap};
use gcd::Gcd;

fn main() {
    let input = read_file();
    let test_input = "LR

                            11A = (11B, XXX)
                            11B = (XXX, 11Z)
                            11Z = (11B, XXX)
                            22A = (22B, XXX)
                            22B = (22C, 22C)
                            22C = (22Z, 22Z)
                            22Z = (22B, 22B)
                            XXX = (XXX, XXX)";

    part2(&input);
}

fn read_file() -> String {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    contents
}

fn part1(input: &str) {
    // WONT WORK DUE TO CHAGNES TO Tree

    let parts: Vec<&str> = input.split("\n\n").collect();
    let instructions = parts[0];

    let mut tree = Tree::new();
    tree.build_tree(parts[1].to_string());

    let mut current_node:isize = 0; // tree.root; 
    let mut is_zzz = false;
    let mut steps:usize = 0;

    while !is_zzz {
        for step in instructions.chars() {
            // print!("From {} go ", current_node);

            if step == 'L' {
                current_node = tree.nodes[current_node as usize].left;
            } else {
                current_node = tree.nodes[current_node as usize].right;
            }

            steps += 1;

            if tree.end_indexes.contains(&current_node) {
                is_zzz = true;
                break;
            }

            // println!("to {}", current_node);
        }
    }

    println!("Steps: {}", steps);
}

fn part2(input: &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let instructions = parts[0];

    let mut tree = Tree::new();
    tree.build_tree(parts[1].to_string());

    let mut small_steps:Vec<usize> = Vec::new();
    
    for i in tree.root_indexes.iter(){
        small_steps.push(get_steps_to_end(&tree, *i, instructions));
    }

    println!("{:?}", small_steps);
    
    let steps:usize = lcm(small_steps);

    println!("Steps: {}", steps);
}

fn get_steps_to_end(tree:&Tree, starting_node:isize, instructions:&str) -> usize{
    let mut current_node:isize = starting_node;
    let mut is_end = false;
    let mut steps:usize = 0;

    while !is_end {
        for step in instructions.chars() {
            // print!("From {} go ", current_node);

            if step == 'L' {
                current_node = tree.nodes[current_node as usize].left;
            } else {
                current_node = tree.nodes[current_node as usize].right;
            }

            steps += 1;
            if tree.end_indexes.contains(&current_node) {
                is_end = true;
                break;
            }

            // println!("to {}", current_node);
        }
    }

    steps
}

fn lcm(numbers: Vec<usize>) -> usize {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        result = result * numbers[i] / result.gcd(numbers[i]);
    }
    result
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
    root_indexes: Vec<isize>,
    end_indexes: Vec<isize>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: Vec::new(),
            root_indexes: Vec::new(),
            end_indexes: Vec::new(),
        }
    }

    fn add_node(&mut self, node_name: String) -> isize{
        self.nodes.push(Node::new(node_name));

        self.nodes.len() as isize - 1
    }

    fn add_children(&mut self, node_index:usize, left: isize, right: isize) {
        if let Some(node) = self.nodes.get_mut(node_index as usize) {
            node.left = left;
            node.right = right;
        }
    }


    fn build_tree(&mut self, nodes_str:String) {
        let mut raw_nodes: HashMap<String, Vec<String>> = HashMap::new();
        let mut node_indexes: HashMap<String, usize> = HashMap::new();

        for node_data in nodes_str.lines() {
            let node_parts: Vec<&str> = node_data.trim().split(" = ").collect();

            // println!("{:?}", node_parts);
            
            let node_name = node_parts[0].trim();
            let node_children = node_parts[1].replace("(","")
                                                            .replace(")","")
                                                            .trim()
                                                            .split(", ")
                                                            .map(|s| s.trim().to_string())
                                                            .collect();

            raw_nodes.insert(node_name.to_string(), node_children);

            let node_index = self.add_node(node_name.to_string());
            node_indexes.insert(node_name.to_string(), node_index as usize);

            if &node_name.chars().last() == &Some('A') {
                self.root_indexes.push(node_index);
            }

            if &node_name.chars().last() == &Some('Z') {
                self.end_indexes.push(node_index);
            }
        }

        for (node_name, node_children) in raw_nodes.iter() {
            let node_index = node_indexes.get(node_name).unwrap();

            let left = node_indexes.get(&node_children[0]).unwrap();
            let right = node_indexes.get(&node_children[1]).unwrap();

            self.add_children(*node_index as usize, *left as isize, *right as isize);
        }

        // self.root = *node_indexes.get("AAA").unwrap() as isize; // PART 1
    }



}

#[derive(Debug)]
struct Node {
    name: String,
    left: isize,
    right: isize,
}
impl Node {
    fn new(name:String) -> Node {
        Node { name, left: -1, right: -1 }
    }


}