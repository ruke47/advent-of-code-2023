use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::time::SystemTime;

struct Puzzle {
    damaged_counts: Vec<usize>,
    data: String,
}

impl Puzzle {
    fn from_line(line: &str) -> Puzzle {
        let parts: Vec<&str> = line.split(" ").collect();
        let corrupted_data = String::from(parts[0]);
        let damaged_groups = parts[1]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        Puzzle {
            damaged_counts: damaged_groups,
            data: corrupted_data,
        }
    }

    fn from_line_but_worse(line: &str) -> Puzzle {
        let parts: Vec<&str> = line.split(" ").collect();
        let mut corrupted_data = String::from(parts[0]);
        for _ in 0..4 {
            corrupted_data.push('?');
            corrupted_data.push_str(parts[0]);
        }

        let base_groups: Vec<usize> = parts[1]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        let mut damaged_groups: Vec<usize> = vec![];
        for _ in 0..5 {
            base_groups.iter().for_each(|n| damaged_groups.push(*n));
        }
        Puzzle {
            damaged_counts: damaged_groups,
            data: corrupted_data,
        }
    }

    fn possible_combos(&self) -> usize {
        let broken_sum: usize = self.damaged_counts.iter().sum();
        // figure out how much wiggle-room each data-line has, overall
        // if it's 3-long and has (1,1), it has 0 wiggle room, it must be #.#
        // if it's 5-long and has (2,1), you need 3 #'s and 1 . between them, leaving 1 . floating
        // it goes either at the front, in the middle, or at the end
        let floating_size = self.data.len() - broken_sum - (self.damaged_counts.len() - 1);
        let group_positions = self.damaged_counts.len() + 1;

        let empty = vec![];
        self.combinations(floating_size, group_positions, &empty)
            .into_iter()
            // .filter(|undamaged_counts| self.prefix_works(undamaged_counts))
            .count()
    }

    fn combinations(&self, sum:usize, groups: usize, so_far: &Vec<usize>) -> Vec<Vec<usize>> {
        // base case: if you're trying to count to 0 using N numbers, all N of them are 0
        if sum == 0 {
            let mut new_vec = so_far.clone();
            for _ in 0..groups {
                new_vec.push(0);
            }
            return if self.prefix_works(&new_vec, true) { vec![new_vec] } else { vec![] };
        }
        // base case: if you're trying to count to N using 1 group, it's N
        if groups == 1 {
            let mut new_vec = so_far.clone();
            new_vec.push(sum);
            return if self.prefix_works(&new_vec, true) { vec![new_vec] } else { vec![] };
        }
        (0..=sum).into_iter()
            .flat_map(|i| {
                let mut new_vec = so_far.clone();
                new_vec.push(i);
                return if self.prefix_works(&new_vec, false) {
                    self.combinations(sum - i, groups - 1, &new_vec).into_iter()
                } else {
                    vec![].into_iter()
                }
            })
            .collect()
    }

    fn prefix_works(&self, undamaged_counts: &Vec<usize>, contains_final: bool) -> bool {
        let mut data_iter = self.data.chars();
        for _ in 0..undamaged_counts[0] {
            let char = data_iter.next().unwrap();
            if char != '.' && char != '?' {
                return false;
            }
        }
        self.damaged_counts.iter().zip(undamaged_counts.iter().skip(1))
            .enumerate()
            .map(|(idx, (dc, udc))| {
                // if this is not the final damaged-count, or this list does not contain
                // the final damaged-count, it actually needs to be 1 larger
                if !contains_final || idx < (self.damaged_counts.len() - 1) {
                    (*dc, *udc + 1)
                } else {
                    (*dc, *udc)
                }
            })
            .all(|(dc, udc)| {
                for _ in 0..dc {
                    let char = data_iter.next().unwrap();
                    if char != '#' && char != '?' {
                        return false;
                    }
                }
                for _ in 0..udc {
                    let char = data_iter.next().unwrap();
                    if char != '.' && char != '?' {
                        return false;
                    }
                }
                return true;
            })
    }
}

fn factorial(num: usize) -> u128 {
    (1..=(num as u128)).product()
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let possibilities: usize = read_puzzles("input", false).iter()
        .map(|p| {
            let combos = p.possible_combos();
            println!("{combos}");
            combos
        })
        .sum();

    println!("Part 1: {possibilities}");
}

fn part2() {
    let begin = SystemTime::now();
    let possibilities: usize = read_puzzles("input", true).iter()
        .map(|p| {
            let combos = p.possible_combos();
            println!("{combos}");
            combos
        })
        .sum();
    let end = SystemTime::now();

    println!("Part 2: {possibilities} in {:.3} seconds",
             end.duration_since(begin).unwrap().as_secs_f32());
}

fn read_puzzles(filename: &str, funky_mode: bool) -> Vec<Puzzle> {
    fs::read_to_string(filename).unwrap()
        .lines()
        .map(if funky_mode {
            Puzzle::from_line_but_worse
        } else {
            Puzzle::from_line
        })
        .collect()
}
