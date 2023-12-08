use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LINE_REGEX: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from_line(line: &str) -> Node {
        let (_, [name, left, right]) = LINE_REGEX
            .captures_iter(line).next()
            .unwrap().extract();

        Node { name: String::from(name), left: String::from(left), right: String::from(right) }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (instructions, nodes) = read_file("input");
    let steps = find_distance(&instructions, &nodes, "AAA", "ZZZ");
    println!("Part 1: {steps}");
}

fn find_distance(instructions: &str, nodes: &HashMap<String, Node>,
                 start_node: &str, end_pattern: &str) -> u64 {
    let mut cur_node = nodes.get(start_node).unwrap();
    let mut steps = 0;
    let mut dir_iter = instructions.chars().cycle();

    loop {
        if cur_node.name.ends_with(end_pattern) {
            break;
        }
        steps += 1;
        match dir_iter.next().unwrap() {
            'L' => cur_node = nodes.get(cur_node.left.as_str()).unwrap(),
            'R' => cur_node = nodes.get(cur_node.right.as_str()).unwrap(),
            _ => panic!("Bad instruction")
        }
    }

    steps
}

fn part2() {
    let (instructions, nodes) = read_file("input");
    let step_counts = nodes.values()
        .filter(|n| n.name.ends_with("A"))
        .map(|n| n.name.as_str())
        .map(|start_name| find_distance(&instructions, &nodes, start_name, "Z"))
        .collect_vec();
    println!("Part 2 Counts: {:?}", step_counts);

    let lcm = step_counts.iter()
        .map(|i| i.clone())
        .reduce(|a, b| lcm(a, b))
        .unwrap();
    println!("Part 2: {lcm}");
}

fn read_file(filename: &str) -> (String, HashMap<String, Node>) {
    let mut parts = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect_vec();
    let nodes: HashMap<String, Node> = parts.pop()
        .unwrap()
        .lines()
        .map(Node::from_line)
        .map(|n| (n.name.clone(), n))
        .collect();
    let instructions = parts.pop().unwrap();

    (instructions, nodes)
}

// shamelessly stolen from https://rustp.org/number-theory/lcm/
fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64{
    // LCM = a*b / gcd
    return a * ( b / gcd(a, b));
}
