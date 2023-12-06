use std::ops::Range;
use std::cmp::Ord;
use itertools::{Itertools};
use d05::load;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (seeds, maps) = load("input");

    // for each seed, do the layered transforms in-order
    let transformed_seeds: Vec<i64> = seeds.iter()
        .map(|seed| maps.iter()
            .fold(seed.clone(),
                  |input, dirtmap| dirtmap.transform(&input)
            ))
        .collect();

    let min_seed = transformed_seeds.iter().min().unwrap();

    println!("Part 1: {min_seed}");
}

fn part2() {
    let (seeds, maps) = load("input");

    // parse the seeds in to chunks of 2
    let seed_ranges: Vec<Range<i64>> = seeds.iter()
        .chunks(2)
        .into_iter()
        .map(|c| c.collect_vec())
        // the first seed is a starting position and the second seed is a length
        .map(|c| Range {start: *c[0], end: c[0] + c[1]})
        .collect();

    // we're going to process all of the ranges at our current layer
    // before moving onto the next layer
    let mut layer_ranges = seed_ranges.clone();

    for layer in maps {
        // for each range in our current layer...
        let output_ranges: Vec<Range<i64>> = layer_ranges.iter()
            // ... transform it into next-layer ranges ...
            .map(|range| layer.to_output_ranges(range))
            // ... then collapse the iter<Vec<Range<> into an iter<Range<>...
            .flatten()
            // ... and collect the iter<Range<> into a Vec<Range<>.
            .collect();
        layer_ranges = output_ranges;
    }

    let smallest_range_begin = layer_ranges.iter()
        .map(|r| r.start)
        .min()
        .unwrap();

    println!("Part 2: {smallest_range_begin}");
}

