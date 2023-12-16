use std::collections::{HashMap, HashSet};
use std::fs;
use crate::Tile::{*};
use crate::Direction::{*};

type Point = (i32, i32);
struct Game {
    board: HashMap<Point, Tile>
}

impl Game {
    fn run(&self) -> usize {
        self.run_starting_at((-1, 0), Right)
    }

    fn run_starting_at(&self, start_point: Point, start_direction: Direction) -> usize {
        // get the unique (Point, Direction) pairs
        let mut visited = HashSet::new();
        self.run_from(start_point, start_direction, &mut visited);

        // filter down to just the unique points
        let squares: HashSet<Point> = visited.iter()
            .map(|(point, _)| *point)
            .collect();

        // return that size
        squares.len()
    }

    // visited is (entered-point, entered-direction)
    fn run_from(&self, start_point: Point,
                start_direction: Direction,
                visited: &mut HashSet<(Point, Direction)>) {
        let mut cur_point = start_point;
        let mut cur_direction = start_direction;
        loop {
            let (dx, dy) = Self::direction_deltas(cur_direction);
            cur_point = (cur_point.0 + dx, cur_point.1 + dy);

            // if we've moved off the board, we're done
            if !self.board.contains_key(&cur_point) {
                break
            }

            // if we've visited a point that we've already visited, we've hit a loop
            let new_visit = visited.insert((cur_point, cur_direction));
            if !new_visit {
                break
            }

            // figure out where we're going next
            let (new_direction, new_split) = Self::new_direction(
                *self.board.get(&cur_point).unwrap(),
                cur_direction);

            // if we've been split, run out the clock on that, then move in the other direction
            if let Some(split_dir) = new_split {
                self.run_from(cur_point, split_dir, visited);
            }
            cur_direction = new_direction;
        }
    }

    fn direction_deltas(direction: Direction) -> Point {
        match direction {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0)
        }
    }

    fn new_direction(hit_tile: Tile, travel_dir: Direction) -> (Direction, Option<Direction>) {
        match hit_tile {
            Blank => (travel_dir, None),
            // /
            MirrorF => {
                match travel_dir {
                    Up => (Right, None),
                    Down => (Left, None),
                    Left => (Down, None),
                    Right => (Up, None)
                }
            }
            // \
            MirrorB => {
                match travel_dir {
                    Up => (Left, None),
                    Down => (Right, None),
                    Left => (Up, None),
                    Right => (Down, None)
                }
            }
            // -
            SplitterH => {
                match travel_dir {
                    Left|Right => (travel_dir, None),
                    Up|Down => (Left, Some(Right))
                }
            }
            // |
            SplitterV => {
                match travel_dir {
                    Up|Down => (travel_dir, None),
                    Left|Right => (Up, Some(Down))
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Tile {
    Blank,
    MirrorF,
    MirrorB,
    SplitterH,
    SplitterV
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let board = load_board("input");
    let visited = board.run();
    println!("Part 1: {}", visited);
}

fn part2() {
    let game = load_board("input");
    let max_x = game.board.keys()
        .map(|(x, _)| *x)
        .max()
        .unwrap();
    let max_y = game.board.keys()
        .map(|(_, y)| *y)
        .max()
        .unwrap();
    let mut entries: Vec<(Point, Direction)> = vec![];
    for &edge in game.board.keys() {
        let (x, y) = edge;
        if x == 0 {
            entries.push(((x - 1, y), Right));
        }
        if x == max_x {
            entries.push(((x + 1, y), Left));
        }
        if y == 0 {
            entries.push(((x, y - 1), Down));
        }
        if y == max_y {
            entries.push(((x, y + 1), Up));
        }
    }

    let max_coverage = entries.iter()
        .map(|(point, dir)| game.run_starting_at(*point, *dir))
        .max()
        .unwrap();

    println!("Part 2: {max_coverage}");
}

fn load_board(filename: &str) -> Game {
    let board = fs::read_to_string(filename).unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    let tile = match ch {
                        '.' => Blank,
                        '/' => MirrorF,
                        '\\' => MirrorB,
                        '-' => SplitterH,
                        '|' => SplitterV,
                        _ => panic!("Unrecognized tile: {ch}")
                    };
                    ((x as i32, y as i32), tile)
                })
        })
        .collect();
    Game {board}
}
