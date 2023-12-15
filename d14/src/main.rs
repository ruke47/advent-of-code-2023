use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::num::Wrapping;
use std::rc::Rc;
use itertools::Itertools;

type Coord = i32;
type Point = (Coord, Coord);

struct Board {
    rocks: HashSet<Point>,
    blocks: Rc<HashSet<Point>>,
    height: Coord,
    width: Coord
}

impl Board {
    fn clone_with(&self, rocks: HashSet<Point>) -> Board {
        Board {
            height: self.height,
            width: self.width,
            blocks: Rc::clone(&self.blocks),
            rocks
        }
    }
    fn roll_n(&self) -> Board {
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
        self.clone_with(new_rocks)
    }


    // forgive me for my sins...
    fn roll_s(&self) -> Board {
        let xs: HashSet<Coord> = self.rocks.iter()
            .map(|r| r.0)
            .collect();
        let mut new_rocks: HashSet<Point> = HashSet::new();
        for x in xs {
            let mut blocks: VecDeque<Coord> = self.blocks.iter()
                .filter(|b| b.0 == x)
                .map(|b| b.1)
                .sorted()
                .rev()
                .collect();
            let rocks: Vec<&Point> = self.rocks.iter()
                .filter(|r| r.0 == x)
                .sorted_by(|a, b| Ord::cmp(a, b))
                .rev()
                .collect();
            let mut y: Coord = self.height - 1;
            for rock in rocks {
                while blocks.front().map(|b| *b > rock.1).unwrap_or(false) {
                    y = blocks[0] - 1;
                    blocks.pop_front();
                }
                while blocks.front().map(|b| *b == y).unwrap_or(false) {
                    y -= 1;
                    blocks.pop_front();
                }
                new_rocks.insert((x, y));
                y -= 1;
            }
        }
        self.clone_with(new_rocks)
    }

    fn roll_w(&self) -> Board {
        let ys: HashSet<Coord> = self.rocks.iter()
            .map(|r| r.1)
            .collect();
        let mut new_rocks: HashSet<Point> = HashSet::new();
        for y in ys {
            let mut blocks: VecDeque<Coord> = self.blocks.iter()
                .filter(|b| b.1 == y)
                .map(|b| b.0)
                .sorted()
                .collect();
            let rocks: Vec<&Point> = self.rocks.iter()
                .filter(|r| r.1 == y)
                .sorted_by(|a, b| Ord::cmp(a, b))
                .collect();
            let mut x: Coord = 0;
            for rock in rocks {
                while blocks.front().map(|b| *b < rock.0).unwrap_or(false) {
                    x = blocks[0] + 1;
                    blocks.pop_front();
                }
                while blocks.front().map(|b| *b == x).unwrap_or(false) {
                    x += 1;
                    blocks.pop_front();
                }
                new_rocks.insert((x, y));
                x += 1;
            }
        }
        self.clone_with(new_rocks)
    }

    fn roll_e(&self) -> Board {
        let ys: HashSet<Coord> = self.rocks.iter()
            .map(|r| r.1)
            .collect();
        let mut new_rocks: HashSet<Point> = HashSet::new();
        for y in ys {
            let mut blocks: VecDeque<Coord> = self.blocks.iter()
                .filter(|b| b.1 == y)
                .map(|b| b.0)
                .sorted()
                .rev()
                .collect();
            let rocks: Vec<&Point> = self.rocks.iter()
                .filter(|r| r.1 == y)
                .sorted_by(|a, b| Ord::cmp(a, b))
                .rev()
                .collect();
            let mut x: Coord = self.width - 1;
            for rock in rocks {
                while blocks.front().map(|b| *b > rock.0).unwrap_or(false) {
                    x = blocks[0] - 1;
                    blocks.pop_front();
                }
                while blocks.front().map(|b| *b == x).unwrap_or(false) {
                    x -= 1;
                    blocks.pop_front();
                }
                new_rocks.insert((x, y));
                x -= 1;
            }
        }
        self.clone_with(new_rocks)
    }

    fn cycle(&self) -> Board {
        self.roll_n().roll_w().roll_s().roll_e()
    }

    fn score(&self) -> Coord {
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

impl PartialEq<Self> for Board {
    fn eq(&self, other: &Self) -> bool {
        self.rocks.eq(&other.rocks)
    }
}

impl Eq for Board {}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // take a hash of the current contents of the Rocks set
        // it won't change, and everything else in the Board is constant
        let mut sum = Wrapping::default();
        self.rocks.iter()
            .for_each(|value| {
                let mut hasher = DefaultHasher::new();
                Hash::hash(&value, &mut hasher);
                sum += hasher.finish();
            });
        state.write_u64(sum.0);
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let board = load_board("input");
    let new_board = board.roll_n();
    // println!("{}", board.to_string());
    println!("Part 1: {}", new_board.score());
}

fn part2() {
    let board = load_board("input");
    let mut cur_board = Rc::new(board);
    let mut count = 0;
    let mut boards_by_count: Vec<Rc<Board>> = vec![Rc::clone(&cur_board)];
    let mut count_map: HashMap<Rc<Board>, usize> = HashMap::from([(Rc::clone(&cur_board), 0)]);
    loop {
        count += 1;
        cur_board = Rc::new(cur_board.cycle());
        if count_map.contains_key(&cur_board) {
            // A B C D E F C D E F C  D  E  F  C  D  E  F
            // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17
            //             ^
            // Cycle Detected at 6, cycle length = 2 - 6 = 4
            // 17 - 2 = 15, 15 % 4 = 3, 17 ~= 2 + 3 = 5 (F)
            let cycle_start = count_map.get(&cur_board).unwrap();
            let cycle_length = count - cycle_start;
            let remaining_length = 1_000_000_000 - cycle_start;
            let equivalent_length = remaining_length % cycle_length;
            let equivalent_board = &boards_by_count[cycle_start + equivalent_length];

            println!("Part 2: {}", equivalent_board.score());
            break
        }
        boards_by_count.push(Rc::clone(&cur_board));
        count_map.insert(Rc::clone(&cur_board), count);
    }

}

fn load_board(filename: &str) -> Board {
    let rocks = get_points(filename, 'O');
    let blocks = get_points(filename, '#');
    let (height, width) = get_dims(filename);
    Board {rocks, blocks: Rc::new(blocks), height, width}
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
