use std::fs;
use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn part1() {
    let vecs = read_file("input");
    let mut sum = 0;
    for vec in vecs {
        sum += extrapolate(vec);
    }

    println!("Part 1: {sum}");
}

fn part2() {
    let vecs = read_file("input");
    let mut sum = 0;
    for vec in vecs {
        sum += destrapolate(vec);
    }

    println!("Part 2: {sum}");
}

fn read_file(filename: &str) -> Vec<Vec<i64>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn extrapolate(line: Vec<i64>) -> i64 {
    let stack = build_stack(line);

    stack.iter()
        .map(|layer| layer.last().unwrap().clone())
        .reduce(|a, b| a + b)
        .unwrap()
}

fn destrapolate(line: Vec<i64>) -> i64 {
    let stack = build_stack(line);

    stack.iter()
        .map(|layer| layer.first().unwrap().clone())
        .rev()
        .reduce(|a, b| b - a)
        .unwrap()
}

fn build_stack(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut stack = vec![line];
    loop {
        let top = stack.last().unwrap();
        if top.iter().all(|n| *n == 0) {
            break;
        }
        let new_layer = top.iter()
            .zip(top.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();
        stack.push(new_layer);
    }
    stack
}

#[cfg(test)]
mod tests {
    use crate::{destrapolate, extrapolate};

    #[test]
    fn extrapolate1() {
        let vec1 = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(extrapolate(vec1), 28);
    }

    #[test]
    fn extrapolate2() {
        let vec2 = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate(vec2), 68);
    }

    #[test]
    fn destrapolat1() {
        let vec1 = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(destrapolate(vec1), 5);
    }
}
