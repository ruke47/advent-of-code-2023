use std::fs;

struct Puzzle {
    damaged_counts: Vec<usize>,
    data: String
}

impl Puzzle {
    fn from_line(line: &str) -> Puzzle {
        let parts: Vec<&str> = line.split(" ").collect();
        let corrupted_data = String::from(parts[0]);
        let damaged_groups = parts[1]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        Puzzle { damaged_counts: damaged_groups, data: corrupted_data }
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
        Puzzle { damaged_counts: damaged_groups, data: corrupted_data }
    }

    fn possible_combos(&self) -> usize {
        let broken_sum: usize = self.damaged_counts.iter().sum();
        // figure out how much wiggle-room each data-line has, overall
        // if it's 3-long and has (1,1), it has 0 wiggle room, it must be #.#
        // if it's 5-long and has (2,1), you need 3 #'s and 1 . between them, leaving 1 . floating
        // it goes either at the front, in the middle, or at the end
        let floating_size = self.data.len() - broken_sum - (self.damaged_counts.len() - 1);
        let group_positions = self.damaged_counts.len() + 1;
        let combos: Vec<Vec<usize>> = combinations(floating_size, group_positions)
            .into_iter()
            .map(|mut v| {
                for i in 1..v.len() {
                    v[i] += 1;
                }
                v
            })
            .collect();

        combos.iter()
            .map(|combo| self.string_with_undamaged(combo))
            .filter(|undamaged| self.matches_data(undamaged))
            .count()
    }

    fn string_with_undamaged(&self, undamaged_counts: &Vec<usize>) -> String {
        let mut data = String::new();
        for group_id in 0..self.damaged_counts.len() {
            for _ in 0..undamaged_counts[group_id] {
                data.push('.');
            }
            for _ in 0..self.damaged_counts[group_id] {
                data.push('#');
            }
        }
        for _ in 0..*undamaged_counts.last().unwrap() {
            data.push('.');
        }

        data
    }

    fn matches_data(&self, repaired_data: &str) -> bool {
        self.data.chars().zip(repaired_data.chars())
            .all(|(ca, cb)| ca == cb || ca == '?')
    }
}

fn combinations(sum:usize, groups: usize) -> Vec<Vec<usize>> {
    if sum == 0 {
        return vec![vec![0; groups]];
    }
    if groups == 1 {
        return vec![vec![sum]];
    }
    (0..=sum).into_iter()
        .flat_map(|i| {
            combinations(sum - i, groups - 1).into_iter()
                .map(move |mut v| {
                    v.push(i);
                    v
                })
        })
        .collect()
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
        .map(|p| p.possible_combos())
        .sum();

    println!("Part 1: {possibilities}");
}

fn part2() {
    let possibilities: usize = read_puzzles("example", true).iter()
        .map(|p| p.possible_combos())
        .sum();

    println!("Part 2: {possibilities}");
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
