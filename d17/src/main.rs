use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::{Hash};
use lib2d::{corners, Point2d};
use crate::Direction::{*};

// TODO: Djikstra's, but for each node, keep track of lowest score for
//       each of 0, 1, 2, 3 straight moves to get there?
//       Think of the space as a 3d-ish grid, where it's better to be lower?

struct Game {
    map: HashMap<Point2d<i32>, i32>,
    max_streak: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Tile {
    point: Point2d<i32>,
    direction: Direction,
    consecutive_steps: i32,
}

impl Tile {
    fn self_and_worse(&self, game: &Game) -> Vec<Tile> {
        let mut siblings = vec![];
        for i in self.consecutive_steps..=game.max_streak {
            let mut sibling = self.clone();
            sibling.consecutive_steps = i;
            siblings.push(sibling);
        }
        siblings
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CostedTile {
    tile: Tile,
    cost: i32
}

impl CostedTile {
    fn new(point: Point2d<i32>,
           direction: Direction,
           consecutive_steps: i32,
           cost: i32) -> CostedTile {
        CostedTile {
            tile: Tile { point, direction, consecutive_steps },
            cost
        }

    }
    fn try_travel(&self, direction: Direction, game: &Game) -> Option<CostedTile> {
        if direction == opposite(self.tile.direction) {
            return None
        }
        if direction == self.tile.direction && self.tile.consecutive_steps == game.max_streak {
            return None;
        }
        let neighbor_point = self.tile.point + delta(direction);
        let neighbor_cost = *game.map.get(&neighbor_point)?;
        let new_consecutive_steps = if direction == self.tile.direction {
            self.tile.consecutive_steps + 1
        } else {
            1
        };
        let new_tile = Tile {
            point: neighbor_point,
            direction,
            consecutive_steps: new_consecutive_steps,
        };
        let new_cost = self.cost + neighbor_cost;
        Some(CostedTile {tile: new_tile, cost: new_cost})
    }
}

impl Ord for CostedTile {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.tile.cmp(&other.tile))
    }
}

impl PartialOrd for CostedTile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    fn find_path(&self) -> i32 {
        let (_, target_point) = corners(self.map.keys()).unwrap();
        let mut explored: HashSet<Tile> = HashSet::new();
        let starting_tile = CostedTile::new(Point2d::new(0, 0), Right, 0, 0);
        let mut to_explore: BinaryHeap<CostedTile> = BinaryHeap::from([starting_tile]);
        while let Some(cur_pos) = to_explore.pop() {
            if explored.contains(&cur_pos.tile) {
                continue
            }
            if cur_pos.tile.point == target_point {
                return cur_pos.cost
            }
            for tile in cur_pos.tile.self_and_worse(self) {
                explored.insert(tile);
            }
            [Up, Down, Left, Right].into_iter()
                .flat_map(|dir| cur_pos.try_travel(dir, self).into_iter())
                .filter(|ct| !explored.contains(&ct.tile))
                .for_each(|ct| to_explore.push(ct));
        }
        panic!("Never found my way to El Dorado");
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    Up, Down, Left, Right
}

fn delta(direction: Direction) -> Point2d<i32> {
    match direction {
        Up => Point2d::new(0, -1),
        Down => Point2d::new(0, 1),
        Left => Point2d::new(-1, 0),
        Right => Point2d::new(1, 0),
    }
}

fn opposite(direction: Direction) -> Direction {
    match direction {
        Up => Down,
        Down => Up,
        Left => Right,
        Right => Left,
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let game = load_map("input", 3);
    let score = game.find_path();
    println!("Part 1: {score}");
}

fn part2() {

}

fn load_map(filename: & str, max_streak: i32) -> Game {
    let map = fs::read_to_string(filename).unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    let val = ch.to_digit(10).unwrap() as i32;
                    (Point2d::new(x as i32, y as i32), val)
                })
        })
        .collect();
    Game { map, max_streak }
}
