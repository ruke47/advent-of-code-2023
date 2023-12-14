use std::collections::{HashSet, VecDeque};
use std::fs;
use itertools::Itertools;

type Coord = i32;
type Point = (Coord, Coord);

struct Board {
    rocks: HashSet<Point>,
    blocks: HashSet<Point>,
    height: Coord,
    width: Coord
}

impl Board {
    fn roll_y(&mut self) {
        let xs: HashSet<Coord> = self.rocks.iter()
            .map(|r| r.0)
            .collect();
        let mut new_rocks: HashSet<Point> = HashSet::new();
        for x in xs {
            let mut blocks: VecDeque<Coord> = self.blocks.iter()
                .filter(|b| b.0 == x)
                .map(|b| b.1)
                .sorted()
                .collect();
            let rocks: Vec<&Point> = self.rocks.iter()
                .filter(|r| r.0 == x)
                .sorted_by(|a, b| Ord::cmp(a, b))
                .collect();
            let mut y: Coord = 0;
            for rock in rocks {
                while blocks.front().map(|b| *b < rock.1).unwrap_or(false) {
                    y = blocks[0] + 1;
                    blocks.pop_front();
                }
                while blocks.front().map(|b| *b == y).unwrap_or(false) {
                    y += 1;
                    blocks.pop_front();
                }
                new_rocks.insert((x, y));
                y += 1;
            }
        }
        self.rocks = new_rocks;
    }

    fn score_y(&self) -> Coord {
        self.rocks.iter()
            .map(|(_, ry)| self.height - ry)
            .sum()
    }

    fn to_string(&self) -> String {
        let mut string = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.rocks.contains(&(x, y)) {
                    string.push('O');
                } else if self.blocks.contains(&(x, y)) {
                    string.push('#');
                } else {
                    string.push('.');
                }
            }
            string.push('\n');
        }

        string
    }
}



fn main() {
    part1();
}

fn part1() {
    let mut board = load_board("input");
    board.roll_y();
    // println!("{}", board.to_string());
    println!("Part 1: {}", board.score_y());
}

fn load_board(filename: &str) -> Board {
    let rocks = get_points(filename, 'O');
    let blocks = get_points(filename, '#');
    let (height, width) = get_dims(filename);
    Board {rocks, blocks, height, width}
}

fn get_points(filename: &str, target: char) -> HashSet<Point> {
    fs::read_to_string(filename).unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(x, ch)| {
                    if ch == target {
                        Some((x as Coord, y as Coord))
                    } else {
                        None
                    }.into_iter()
                })
        })
        .collect()
}

fn get_dims(filename: &str) -> (Coord, Coord) {
    let text = fs::read_to_string(filename).unwrap();
    let height = text.lines().count() as Coord;
    let width = text.lines().next().unwrap().len() as Coord;

    (height, width)
}
