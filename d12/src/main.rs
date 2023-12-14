use std::fs;
use std::time::SystemTime;
use rayon::prelude::*;

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
    }

    fn combinations(&self, sum:usize, groups: usize, so_far: &Vec<usize>) -> usize {
        // in either base-case, the last item in the vec is the "final" item in the vec;
        // otherwise, more items will be added to the end of the vec

        // base case: if you're trying to count to 0 using N numbers, all N of them are 0
        if sum == 0 {
            let mut new_vec = so_far.clone();
            for _ in 0..groups {
                new_vec.push(0);
                if !self.prefix_works(&new_vec) {
                    return 0
                }
            }
            return 1
        }
        // base case: if you're trying to count to N using 1 group, it's N
        if groups == 1 {
            let mut new_vec = so_far.clone();
            new_vec.push(sum);
            return if self.prefix_works(&new_vec) { 1 } else { 0 };
        }

        (0..=sum).into_iter()
            .map(|i| {
                // see if it works to add 'i' to the end of our list...
                let mut new_vec = so_far.clone();
                new_vec.push(i);
                return if self.prefix_works(&new_vec) {
                    // if so, collect it's working children
                    self.combinations(sum - i, groups - 1, &new_vec)
                } else {
                    // otherwise, this is a dead branch
                    0
                }
            })
            .sum()
    }

    fn prefix_works(&self, undamaged_counts: &Vec<usize>) -> bool {
        let contains_final = undamaged_counts.len() > self.damaged_counts.len();

        // we already know the first (n-1) groups are fine, because we've prefix-tested them
        let groups_to_skip = undamaged_counts.len() - 1;
        // skip the characters in the first n-1 undamaged groups,
        // the first n-1 damaged groups
        // and the extra required undamaged character after each damaged group
        let skip_ahead = undamaged_counts.iter().take(groups_to_skip).sum::<usize>() +
            self.damaged_counts.iter().take(groups_to_skip).sum::<usize>()
            + groups_to_skip
            // if this is the final undamaged count, we didn't require the prior separator
            - if contains_final { 1 } else { 0 };
        let mut data_iter = self.data.chars().skip(skip_ahead);

        for _ in 0..*(undamaged_counts.last().unwrap()) {
            let char = data_iter.next().unwrap();
            if char != '.' && char != '?' {
                return false;
            }
        }
        // if we're not running the final undamaged group, make sure that the damaged group
        // after this damaged group is kosher, including its separator
        if !contains_final {
            for _ in 0..self.damaged_counts[undamaged_counts.len() - 1] {
                let char = data_iter.next().unwrap();
                if char != '#' && char != '?' {
                    return false;
                }
            }
            // if this is not the last damaged group, it must have an extra
            // undamaged separator between it and the next damaged group
            if self.damaged_counts.len() > undamaged_counts.len() {
                let last_char = data_iter.next().unwrap();
                if last_char != '.' && last_char != '?' {
                    return false;
                }
            }
        }
        return true;
    }
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
    let possibilities: usize = read_puzzles("input", true)
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(id, p)| {
            let combos = p.possible_combos();
            println!("{id}: {combos}");
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
