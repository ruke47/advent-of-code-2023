use std::cmp::{max, min};
use std::collections::{HashSet};
use std::fs;

type Point = (usize, usize);

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Galaxy {
    location: Point,
    name: usize,
}

struct Universe {
    present_xs: HashSet<usize>,
    present_ys: HashSet<usize>,
    expansion: usize,
}

impl Galaxy {
    fn distance_to(&self, other: &Galaxy, universe: &Universe) -> usize {
        let min_x = min(self.location.0, other.location.0);
        let min_y = min(self.location.1, other.location.1);
        let max_x = max(self.location.0, other.location.0);
        let max_y = max(self.location.1, other.location.1);
        let x_range = min_x..max_x;
        let y_range = min_y..max_y;

        // the base distance is the sum of the vertical + horizontal distance
        let base_distance = x_range.len() + y_range.len();

        // figure out how many of the base rows in the x & y ranges
        // have a universe in them blocking expansion
        let overlap_x = universe.present_xs.iter()
            .filter(|x| x_range.contains(x))
            .count();
        let overlap_y = universe.present_ys.iter()
            .filter(|y| y_range.contains(y))
            .count();

        // the total distance is the base distance,
        // plus the unblocked rows/cols times the expansion factor
        base_distance + (universe.expansion - 1) * (base_distance - overlap_x - overlap_y)
    }
}

impl Universe {
    fn from(galaxies: &HashSet<Galaxy>, expansion: usize) -> Universe {
        let xs = galaxies.iter()
            .map(|g| g.location.0)
            .collect();
        let ys = galaxies.iter()
            .map(|g| g.location.1)
            .collect();
        Universe {present_xs: xs, present_ys: ys, expansion }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let galaxies = read_file("input");
    let universe = Universe::from(&galaxies, 2);

    let distance_sum = sum_distances(&galaxies, &universe);

    println!("Part 1: {distance_sum}");
}

fn part2() {
    let galaxies = read_file("input");
    let universe = Universe::from(&galaxies, 1_000_000);

    let distance_sum = sum_distances(&galaxies, &universe);

    println!("Part 2: {distance_sum}");
}

fn sum_distances(galaxies: &HashSet<Galaxy>, universe: &Universe) -> usize {
    let distance_sum: usize = galaxies.iter()
        .flat_map(|ga| {
            galaxies.iter()
                .filter(|gb| gb.name > ga.name)
                .map(move |gb| (ga, gb).clone())
        })
        .map(|(ga, gb)| {
            let distance = ga.distance_to(gb, &universe);
            // println!("{} -> {}: {distance}", ga.name, gb.name);
            distance
        })
        .sum();
    distance_sum
}

fn read_file(file: &str) -> HashSet<Galaxy> {
    let mut name = 0;
    fs::read_to_string(file).unwrap()
        .lines().enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .map(move |(x, ch)| {
                    match ch {
                        '#' => Some((x, y).clone()),
                        _ => None
                    }
                })
                .flat_map(|o| o.into_iter())
        })
        .map(|p| {
            name += 1;
            Galaxy {location: p, name}
        })
        .collect()
}
