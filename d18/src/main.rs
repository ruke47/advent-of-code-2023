use std::collections::HashSet;
use std::fs;
use lib2d::{corners, Point2d};
use crate::Direction::*;

struct Instruction {
    direction: Direction,
    length: i64,
    hex_direction: Direction,
    hex_length: i64
}

#[derive(Copy, Hash, Eq, PartialEq, Debug, Clone)]
enum Direction {
    Up, Down, Left, Right
}

fn direction_offset(direction: Direction) -> Point2d<i64> {
    match direction {
        Up => Point2d::new(0, -1),
        Down => Point2d::new(0, 1),
        Left=> Point2d::new(-1, 0),
        Right => Point2d::new(1, 0),
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let instructions = read_instructions("input");
    let dug = dig(&instructions);
    // print_pool(&dug);
    let volume = count_inside_points(&dug);
    println!("Part 1: {volume}");
}

fn part2() {
    let instructions = read_instructions("input");
    let vertices = trace(&instructions);
    // this algorithm stolen from: https://www.mathopenref.com/coordpolygonarea.html
    let area = vertices.iter().zip(vertices.iter().skip(1))
        .map(|(pa, pb)| {
            pa.x * pb.y - pa.y * pb.x
        })
        .sum::<i64>() / 2;

    // I just kinda of guessed at adding the outline, tbh.
    // I knew that my numbers were of because the line lengths are "new distance added",
    // not length of a face.
    // Adding the outline didn't give me the right area, but adding HALF of the outline gave me
    // 1 off from the right number. So I fudged it. I'm SORRY.
    let outline: i64 = instructions.iter()
        .map(|i| i.hex_length)
        .sum();
    let total_area = area + outline / 2 + 1;
    println!("Part 2: {total_area}");
    // Should: 952408144115
    //    Got: 952408144114
}

// instead of collecting all of the points, collect all of the lines that make up this shape
fn trace(instructions: &Vec<Instruction>) -> Vec<Point2d<i64>> {
    let mut cur_point = Point2d::new(0i64, 0i64);
    let mut points = vec![cur_point];
    for instruction in instructions {
        let delta = direction_offset(instruction.hex_direction) * instruction.hex_length;
        cur_point = cur_point + delta;
        points.push(cur_point);
    }
    points
}

// collect the points that make up the dug trench
fn dig(instructions: &Vec<Instruction>) -> HashSet<Point2d<i64>> {
    let mut cur_point = Point2d::new(0, 0);
    let mut dug = HashSet::new();
    dug.insert(cur_point);
    for instruction in instructions {
        let delta = direction_offset(instruction.direction);
        for _ in 0..instruction.length {
            cur_point = cur_point + delta;
            dug.insert(cur_point);
        }
    }

    dug
}

// draw the trench from it's points
fn print_pool(points: &HashSet<Point2d<i64>>) {
    let (min, max) = corners(points.iter()).unwrap();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if points.contains(&Point2d::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

// count the inside points using a "paint" method
fn count_inside_points(points: &HashSet<Point2d<i64>>) -> usize {
    let mut inside = HashSet::new();
    // I know that 1, 1 is inside from looking at the drawing :brain-genious:
    let initial_inside = Point2d::new(1, 1);
    inside.insert(initial_inside);
    let mut to_process = vec![initial_inside];
    while let Some(inside_point) = to_process.pop() {
        [Up, Down, Left, Right].into_iter()
            .map(direction_offset)
            .for_each(|delta| {
                let neighbor = inside_point + delta;
                if !points.contains(&neighbor) {
                    let newly_inserted = inside.insert(neighbor);
                    if newly_inserted {
                        to_process.push(neighbor);
                    }
                }
            });
    }

    inside.len() + points.len()
}

fn read_instructions(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename).unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = match parts[0] {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                other => panic!("Invalid direction: {other}")
            };
            let length = parts[1].parse().unwrap();
            let hex_distance = &parts[2][2..(parts[2].len() - 2)];
            let hex_length = i64::from_str_radix(hex_distance, 16).unwrap();
            let hex_direction = match parts[2].chars().rev().skip(1).next().unwrap() {
                '0' => Right,
                '1' => Down,
                '2' => Left,
                '3' => Up,
                other => panic!("Unexpected hex direction: {other}")
            };

            Instruction {direction, length, hex_direction, hex_length}
        })
        .collect()
}
