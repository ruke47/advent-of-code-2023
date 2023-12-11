use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs;
use crate::Color::{BLUE, RED};
use crate::Direction::{LEFT, RIGHT, UP, DOWN};

type Point = (i32, i32);

#[derive(Debug)]
struct Node {
    location: Point,
    val: char,
    // distance from 'S'
    distance: RefCell<Option<usize>>,
    // imagine painting each "wall" of the 2D shape a color,
    // so we can differentiate "inside" and "outside" later
    colors: RefCell<HashMap<Direction, Color>>,
}

impl Node {
    fn new(location: Point, val: char) -> Self {
        // the 'S' node is 0 distance, the others are an unknown distance
        let distance = if val == 'S' {
            RefCell::new(Some(0))
        } else {
            RefCell::new(None)
        };

        // we need to build the loop before we can start painting the walls
        let colors = RefCell::new(HashMap::new());
        Node { location, val, distance, colors }
    }

    // self is part of the connected loop
    // look at the source node to tell what colors your own walls should be
    fn paint_tube(&self, travel_direction: &Direction, from_node: &Node) {
        let mut own_colors = self.colors.borrow_mut();
        let other_colors = from_node.colors.borrow();
        match self.val {
            // | and - only have 2 colored walls
            '|' => {
                own_colors.insert(LEFT, *other_colors.get(&LEFT).unwrap());
                own_colors.insert(RIGHT, *other_colors.get(&RIGHT).unwrap());
            },
            '-' => {
                own_colors.insert(UP, *other_colors.get(&UP).unwrap());
                own_colors.insert(DOWN, *other_colors.get(&DOWN).unwrap());
            },
            // all other shapes have 4 colored walls
            'L' => {
                let (left_down, right_up) = if *travel_direction == DOWN {
                    (*other_colors.get(&LEFT).unwrap(), *other_colors.get(&RIGHT).unwrap())
                } else {
                    (*other_colors.get(&DOWN).unwrap(), *other_colors.get(&UP).unwrap())
                };

                own_colors.insert(LEFT, left_down);
                own_colors.insert(DOWN, left_down);
                own_colors.insert(RIGHT, right_up);
                own_colors.insert(UP, right_up);
            },
            'J' => {
                let (left_up, right_down) = if *travel_direction == DOWN {
                    (*other_colors.get(&LEFT).unwrap(), *other_colors.get(&RIGHT).unwrap())
                } else {
                    (*other_colors.get(&UP).unwrap(), *other_colors.get(&DOWN).unwrap())
                };

                own_colors.insert(LEFT, left_up);
                own_colors.insert(DOWN, right_down);
                own_colors.insert(RIGHT, right_down);
                own_colors.insert(UP, left_up);
            },
            '7' => {
                let (right_up, left_down) = if *travel_direction == RIGHT {
                    (*other_colors.get(&UP).unwrap(), *other_colors.get(&DOWN).unwrap())
                } else {
                    (*other_colors.get(&RIGHT).unwrap(), *other_colors.get(&LEFT).unwrap())
                };

                own_colors.insert(LEFT, left_down);
                own_colors.insert(DOWN, left_down);
                own_colors.insert(RIGHT, right_up);
                own_colors.insert(UP, right_up);
            },
            'F' => {
                let (left_up, right_down) = if *travel_direction == LEFT {
                    (*other_colors.get(&UP).unwrap(), *other_colors.get(&DOWN).unwrap())
                } else {
                    (*other_colors.get(&LEFT).unwrap(), *other_colors.get(&RIGHT).unwrap())
                };

                own_colors.insert(LEFT, left_up);
                own_colors.insert(DOWN, right_down);
                own_colors.insert(RIGHT, right_down);
                own_colors.insert(UP, left_up);
            },
            _ => panic!("own has bad value: {}", self.val)

        }
    }

    // self is NOT part of the connected loop
    // look to our immediate neighbors - if one of them has a painted wall,
    // paint all of our walls the same color
    fn paint_unconnected(&self, neighbors: Vec<(Direction, &Node)>) -> bool {
        let neighbor_color = neighbors.iter()
            .flat_map(|(direction, neighbor)| {
                // invert the direction - if the neighbor is to our left, look at its right wall
                let inverted_direction = direction.invert();
                neighbor.colors.borrow().get(&inverted_direction)
                    .map(|c| c.clone())
                    .into_iter()
            })
            .next();
        match neighbor_color {
            // if none of our neighbors is painted, we cannot be painted at this time
            None => false,
            Some(color) => {
                let mut colors = self.colors.borrow_mut();
                colors.insert(UP, color);
                colors.insert(DOWN, color);
                colors.insert(LEFT, color);
                colors.insert(RIGHT, color);

                true
            }
        }
    }

    // we are one further from the 'S' node than our neighbor
    fn set_distance(&self, from_node: &Node) {
        let _ = self.distance.borrow_mut().insert(from_node.distance.borrow().unwrap() + 1);
    }

    // only call on the 'S' node
    // figure out the S node's implied shape, than paints all of its walls a consistent color
    // we don't know whether "red" or "blue" will be inside/outside yet,
    // but the streams will never cross
    fn initialize_colors(&self, neighbors: Vec<(Direction, &Node)>) {
        if self.val != 'S' {
            panic!("Only call initialize_colors on the 'S' node");
        }

        let mut self_colors = self.colors.borrow_mut();

        // look to our connected neighbors, and see what directions they are from us
        let mut directions: Vec<Direction> = neighbors.iter()
            .filter(|(direction, neighbor)| can_travel(self, direction, neighbor))
            .map(|t| t.0)
            .collect();
        // sort the directions, so we always get a (UP, DOWN) tuple and never a (DOWN, UP) tuple
        directions.sort();
        let directions = (directions[0], directions[1]);

        match directions {
            (UP, DOWN) => {
                // |
                self_colors.insert(LEFT, BLUE);
                self_colors.insert(RIGHT, RED);
            },
            (UP, LEFT) => {
                // J
                self_colors.insert(UP, BLUE);
                self_colors.insert(DOWN, RED);
                self_colors.insert(LEFT, BLUE);
                self_colors.insert(RIGHT, RED);
            },
            (UP, RIGHT) => {
                // L
                self_colors.insert(UP, BLUE);
                self_colors.insert(DOWN, RED);
                self_colors.insert(LEFT, RED);
                self_colors.insert(RIGHT, BLUE);
            },
            (DOWN, LEFT) => {
                // 7
                self_colors.insert(UP, BLUE);
                self_colors.insert(DOWN, RED);
                self_colors.insert(LEFT, RED);
                self_colors.insert(RIGHT, BLUE);
            },
            (DOWN, RIGHT) => {
                // F
                self_colors.insert(UP, BLUE);
                self_colors.insert(DOWN, RED);
                self_colors.insert(LEFT, BLUE);
                self_colors.insert(RIGHT, RED);
            },
            (LEFT, RIGHT) => {
                // -
                self_colors.insert(UP, BLUE);
                self_colors.insert(DOWN, RED);
            },
            _ => panic!("Can't identify starting block with neighbors {:?}", neighbors)
        };
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn invert(&self) -> Direction{
        match self {
            UP => DOWN,
            DOWN => UP,
            LEFT => RIGHT,
            RIGHT => LEFT
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Color {
    RED, BLUE
}

fn main() {
    let map = read_input("input");

    let start_location = map.values()
        .find(|n| n.val == 'S')
        .unwrap();
    start_location.initialize_colors(get_neighbors(&map, &start_location));

    // by using a queue here, we will guarantee that we are working from the closest two outwards
    // (the front will always contain the node with the smallest distance)
    let mut to_explore = VecDeque::new();
    to_explore.push_back(start_location);

    let mut has_explored = vec![];

    while !to_explore.is_empty() {
        let cur_node = to_explore.pop_front().unwrap();
        for (direction, neighbor) in get_neighbors(&map, &cur_node) {
            if can_travel(&cur_node, &direction, &neighbor) {
                neighbor.set_distance(cur_node);
                neighbor.paint_tube(&direction, cur_node);
                to_explore.push_back(neighbor);
            }
        }
        has_explored.push(cur_node);
    }

    let furthest_distance = has_explored.iter()
        .map(|n| n.distance.borrow().unwrap())
        .max()
        .unwrap();
    println!("Part 1: {furthest_distance}");

    // if a node doesn't have a distance, it's not part of the loop
    let unconnected: Vec<&Node> = map.values()
        .filter(|n| n.distance.borrow().is_none())
        .collect();
    let mut unpainted = unconnected.clone();

    // go through the unpainted nodes, attempting to paint each
    while !unpainted.is_empty() {
        let mut unpaintable: Vec<&Node> = vec![];
        for &node in unpainted.iter() {
            let neighbors = get_neighbors(&map, node);
            // if this node cannot yet be painted, add it to the unpaintable list
            if !node.paint_unconnected(neighbors) {
                unpaintable.push(node);
            }
        }
        // if all of the unpainted nodes we started with are unpaintable,
        // nothing will change the next time through the loop
        if unpaintable.len() == unpainted.len() {
            panic!("Couldn't paint any nodes!");
        }
        unpainted = unpaintable;
    }

    // make sure we have sane red+blue counts
    let blue_count = unconnected.iter()
        .filter(|n| *n.colors.borrow().get(&UP).unwrap() == BLUE)
        .count();
    let red_count = unconnected.iter()
        .filter(|n| *n.colors.borrow().get(&UP).unwrap() == RED)
        .count();
    assert_eq!(red_count + blue_count, unconnected.len());
    println!("Red: {red_count}\nBlue: {blue_count}");

    // I can see a '.' in the top row. It must be outside.
    let outside_color = *unconnected.iter()
        .find(|n| n.location.0 == 0)
        .unwrap()
        .colors.borrow()
        .get(&UP)
        .unwrap();
    println!("Outside is {:?}", outside_color);
}

fn read_input(filename: &str) -> HashMap<Point, Node> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    let point = (x as i32, y as i32);
                    (point.clone(), Node::new(point.clone(), ch))
                })
        })
        .collect()
}

fn can_travel(cur_node: &Node, travel_direction: &Direction, to_node: &Node) -> bool {
    // don't travel to places you've already been
    if to_node.distance.borrow().is_some() {
        return false;
    }

    let to_letter = to_node.val;
    let from_letter = cur_node.val;
    match from_letter {
        '|' => match (travel_direction, to_letter) {
            (UP, '7') => true,
            (UP, 'F') => true,
            (UP, '|') => true,
            (DOWN, 'L') => true,
            (DOWN, 'J') => true,
            (DOWN, '|') => true,
            _ => false
        },
        '-' => match (travel_direction, to_letter) {
            (LEFT, 'L') => true,
            (LEFT, 'F') => true,
            (LEFT, '-') => true,
            (RIGHT, 'J') => true,
            (RIGHT, '7') => true,
            (RIGHT, '-') => true,
            _ => false
        },
        'L' => match (travel_direction, to_letter) {
            (UP, '7') => true,
            (UP, 'F') => true,
            (UP, '|') => true,
            (RIGHT, 'J') => true,
            (RIGHT, '7') => true,
            (RIGHT, '-') => true,
            _ => false,
        },
        'J' => match (travel_direction, to_letter) {
            (UP, '7') => true,
            (UP, 'F') => true,
            (UP, '|') => true,
            (LEFT, 'L') => true,
            (LEFT, 'F') => true,
            (LEFT, '-') => true,
            _ => false
        },
        '7' => match (travel_direction, to_letter) {
            (LEFT, 'L') => true,
            (LEFT, 'F') => true,
            (LEFT, '-') => true,
            (DOWN, 'L') => true,
            (DOWN, 'J') => true,
            (DOWN, '|') => true,
            _ => false
        },
        'F' => match (travel_direction, to_letter) {
            (RIGHT, 'J') => true,
            (RIGHT, '7') => true,
            (RIGHT, '-') => true,
            (DOWN, 'L') => true,
            (DOWN, 'J') => true,
            (DOWN, '|') => true,
            _ => false
        },
        'S' => match (travel_direction, to_letter) {
            (UP, '7') => true,
            (UP, 'F') => true,
            (UP, '|') => true,
            (DOWN, 'L') => true,
            (DOWN, 'J') => true,
            (DOWN, '|') => true,
            (LEFT, 'L') => true,
            (LEFT, 'F') => true,
            (LEFT, '-') => true,
            (RIGHT, 'J') => true,
            (RIGHT, '7') => true,
            (RIGHT, '-') => true,
            _ => false
        },
        _ => false
    }
}

fn get_neighbors<'a>(map: &'a HashMap<Point, Node>, cur_node: &Node) -> Vec<(Direction, &'a Node)> {
    let up_loc = map.get(&(cur_node.location.0, cur_node.location.1 - 1))
        .map(|n| (UP, n));
    let down_loc = map.get(&(cur_node.location.0, cur_node.location.1 + 1))
        .map(|n| (DOWN, n));
    let left_loc = map.get(&(cur_node.location.0 - 1, cur_node.location.1))
        .map(|n| (LEFT, n));
    let right_loc = map.get(&(cur_node.location.0 + 1, cur_node.location.1))
        .map(|n| (RIGHT, n));
    return vec![up_loc, down_loc, left_loc, right_loc].into_iter()
        .flat_map(|o| o.into_iter())
        .collect();
}
