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
    min_movement: i32
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Tile {
    point: Point2d<i32>,
    direction: Direction,
    consecutive_steps: i32,
}

impl Tile {
    // when making note of Tiles that have been visited,
    // (23, 7) RIGHT 1 should prevent us from visiting
    // (23, 7) RIGHT 2, because that's the same position, but with fewer options going forwards
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
        // You may not turn around
        if direction == opposite(self.tile.direction) {
            return None
        }
        // if we've already traveled our max_streak to get here, we can't keep going
        // in the same direction
        if direction == self.tile.direction && self.tile.consecutive_steps >= game.max_streak {
            return None;
        }

        // If we've just turned, we have to go the minimum distance
        // if we're traveling in the same direction, we're allowed to go 1 square at a time
        let move_distance = if direction == self.tile.direction { 1 } else { game.min_movement };
        let mut end_point = self.tile.point;
        let mut move_cost = 0;
        for _ in 0..move_distance {
            end_point = end_point + delta(direction);
            // Note: ? here forces entire function to return None if the point is not in map
            move_cost += game.map.get(&end_point)?;
        }

        // if we're continuing going the same direction,
        // add the previous tile's distance to our own movement.
        // otherwise only count our new movement
        let new_consecutive_steps = if direction == self.tile.direction {
            self.tile.consecutive_steps + move_distance
        } else {
            move_distance
        };

        let new_tile = Tile {
            point: end_point,
            direction,
            consecutive_steps: new_consecutive_steps,
        };

        let new_cost = self.cost + move_cost;
        Some(CostedTile {tile: new_tile, cost: new_cost})
    }
}

// Define "Ordered" for Costed Tile, so we can put them in a Heap
impl Ord for CostedTile {
    fn cmp(&self, other: &Self) -> Ordering {
        // do other.cmp(this), so the smallest comes out on top
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
    let game = load_map("input", 1, 3);
    let score = game.find_path();
    println!("Part 1: {score}");
}

fn part2() {
    let game = load_map("input", 4, 10);
    let score = game.find_path();
    println!("Part 2: {score}");
}

fn load_map(filename: & str, min_movement: i32, max_streak: i32) -> Game {
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
    Game { map, max_streak, min_movement }
}
