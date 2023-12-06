use std::cmp::{min, Ordering};
use std::fs;
use std::ops::Range;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct DirtTransform {
    pub range: Range<i64>,
    pub transform: i64
}

impl DirtTransform {
    pub fn contains(&self, val: &i64) -> bool {
        self.range.contains(val)
    }

    pub fn transform(&self, val: &i64) -> i64 {
        val + self.transform
    }
}

impl PartialOrd for DirtTransform {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirtTransform {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct DirtMap {
    pub name: String,
    pub transforms: Vec<DirtTransform>
}

impl DirtMap {
    pub fn transform(&self, source: &i64) -> i64 {
        self.transforms.iter()
            .find(|xform| xform.contains(&source))
            .map(|xform| xform.transform(&source))
            .unwrap_or(source.clone())
    }

    // take a single input range,
    // split it into ranges that map to my transform's input ranges,
    // and return those ranges, transformed
    pub fn to_output_ranges(&self, input_range: &Range<i64>) -> Vec<Range<i64>> {
        let mut output_ranges = Vec::new();
        let mut cur_range = input_range.clone();
        let mut xform_iter = self.transforms.iter();
        loop {
            // if the range has been consumed, break
            if cur_range.is_empty() {
                break;
            }

            // if we are out of transform ranges, break
            let current_transform = match xform_iter.next() {
                None => break,
                Some(xform) => xform
            };

            // if the current range ends before this transform begins, we're done
            if cur_range.end <= current_transform.range.start {
                break;
            }

            // if the current range begins before the current transform
            if cur_range.start <= current_transform.range.start {
                // split off the first portion as an untransformed range
                let before_range_start = cur_range.start;
                let before_range_end = current_transform.range.start;
                let before_range = Range {start: before_range_start, end: before_range_end};
                if !before_range.is_empty() {
                    output_ranges.push(before_range)
                }

                // split off any overlap as a transformed range
                let inner_range_start = current_transform.range.start;
                let inner_range_end = min(cur_range.end, current_transform.range.end);
                let inner_range = Range {
                    start: current_transform.transform(&inner_range_start),
                    end: current_transform.transform(&inner_range_end)
                };
                if inner_range.is_empty() {
                    panic!("Impossible! Inner range should not be empty");
                }
                output_ranges.push(inner_range);

                // if the current range extends past the transform range,
                // set the dangling bit to be our current range
                cur_range = Range {start: inner_range_end, end: input_range.end};
                continue;
            }

            // if the current transform begins before the current range & contains some of it
            if cur_range.start < current_transform.range.end {
                // add the inner range as a transformed range
                let inner_range_start = cur_range.start;
                let inner_range_end = min(cur_range.end, current_transform.range.end);
                let inner_range = Range {
                    start: current_transform.transform(&inner_range_start),
                    end: current_transform.transform(&inner_range_end)
                };
                if inner_range.is_empty() {
                    panic!("Impossible! Inner range should not be empty")
                }
                output_ranges.push(inner_range);

                // if there's any dangling remained, set it to be our current range
                cur_range = Range {start: inner_range_end, end: cur_range.end};
                continue;
            }
        }

        // if there's any range left over after the last transform, append it untransformed
        if !cur_range.is_empty() {
            output_ranges.push(cur_range);
        }

        return output_ranges;
    }
}


pub fn load(filename: &str) -> (Vec<i64>, Vec<DirtMap>) {
    let file_str = fs::read_to_string(filename)
        .unwrap();
    let mut parts = file_str.split("\n\n");

    let seeds: Vec<i64> = parts.next().unwrap()
        .split(":")
        .last().unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let maps: Vec<DirtMap> = parts
        .map(|block| parse_map(block))
        .collect();

    (seeds, maps)
}

pub fn parse_map(block: &str) -> DirtMap {
    let mut lines = block.lines();
    let name = String::from(lines.next().unwrap());
    let transforms: Vec<DirtTransform> = lines
        .map(parse_transform)
        .sorted()
        .collect();

    DirtMap {name, transforms}
}

pub fn parse_transform(line: &str) -> DirtTransform {
    let parts: Vec<i64> = line.split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    assert_eq!(parts.len(), 3);
    let range = Range {start: parts[1], end: parts[1] + parts[2]};
    let transform = parts[0] - parts[1];

    DirtTransform { range, transform }
}
