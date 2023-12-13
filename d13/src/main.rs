use std::fs;

struct Map {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>
}

impl Map {
    fn from_string(input: &str) -> Map {
        let rows: Vec<Vec<char>> = input.lines()
            .map(|line| line.chars().collect())
            .collect();
        // build a column-centric view of the data so we only have to write 1 search algorithm
        let mut cols = vec![];
        for i in 0..rows[0].len() {
            let col: Vec<char> = rows.iter().map(|row| row[i]).collect();
            cols.push(col);
        }

        Map { rows, cols }
    }

    fn find_mirror_col(&self, smudges: usize) -> Option<usize> {
        Self::find_mirror(&self.cols, smudges)
    }

    fn find_mirror_row(&self, smudges: usize) -> Option<usize> {
        Self::find_mirror(&self.rows, smudges)
    }

    fn find_mirror(elems: &Vec<Vec<char>>, smudges: usize) -> Option<usize> {
        let size = elems.len();
        // for each line that we could reflect about
        for mirror_after in 1..size {
            // zip the list of vectors with itself
            // but one has skipped ahead N
            // and the other is reversed, then skips all of the items in the first list
            let saw_smudges: usize = elems.iter().skip(mirror_after)
                .zip(elems.iter().rev().skip(size - mirror_after))
                // count how many imperfections we see about this line of reflection
                .map(|(a, b)| {
                    Self::count_diffs(a, b)
                })
                .sum();
            // if it matches our goal, return this line
            if saw_smudges == smudges {
                return Some(mirror_after)
            }
        }
        None
    }

    fn count_diffs(a: &Vec<char>, b: &Vec<char>) -> usize {
        a.iter().zip(b.iter())
            .filter(|(a, b)| a == b)
            .count()
    }

}

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum: usize = read_file("input").iter()
        .map(|map| {
            let mc = map.find_mirror_col(0);
            let mr = map.find_mirror_row(0);
            mc.or(mr.map(|n| n * 100)).unwrap()
        })
        .sum();
    println!("Part 1: {sum}");
}

fn part2() {
    let sum: usize = read_file("input").iter()
        .map(|map| {
            let mc = map.find_mirror_col(1);
            let mr = map.find_mirror_row(1);
            mc.or(mr.map(|n| n * 100)).unwrap()
        })
        .sum();
    println!("Part 1: {sum}");
}

fn read_file(filename: &str) -> Vec<Map> {
    fs::read_to_string(filename).unwrap()
        .split("\n\n")
        .map(Map::from_string)
        .collect()
}
