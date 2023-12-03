use std::cmp::max;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static!{
pub static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
pub static ref SYMBOL: Regex = Regex::new(r"[^.\d]").unwrap();
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Label {
    number: u32,
    y: i32,
    xrange: Range,
}

impl Label {
    fn adjacent_to(&self, symbol: &Symbol) -> bool {
        symbol.yrange().contains(self.y) &&
            symbol.xrange().overlaps(&self.xrange)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Symbol {
    value: String,
    x: i32,
    y: i32
}

impl Symbol {
    fn xrange(&self) -> Range {
        Range {
            begin: max(0, self.x - 1),
            end: self.x + 1
        }
    }

    fn yrange(&self) -> Range {
        Range {
            begin: max(0, self.y - 1),
            end: self.y + 1
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Range {
    begin: i32,
    end: i32,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        return (self.begin >= other.begin && self.begin <= other.end) ||
            (other.begin >= self.begin && other.begin <= self.end);
    }

    fn contains(&self, value: i32) -> bool {
        return self.begin <= value && self.end >= value;
    }
}


fn main() {
    part1();
    part2();
}

fn debug() {
    let (labels, symbols) = parse_map("example");
    let matching_labels: Vec<&Label> = labels.iter()
        .filter(|label| {
            symbols.iter().any(|symbol| {
                label.adjacent_to(symbol)
            })
        })
        .collect();
    let label_20 = labels.iter().find(|l| l.number == 20).unwrap();
    let matches_y_0 = symbols[0].yrange().contains(label_20.y);
    let matches_x_0 = symbols[0].xrange().overlaps(&label_20.xrange);
    let matches_y_1 = symbols[1].yrange().contains(label_20.y);
    let matches_x_1 = symbols[1].xrange().overlaps(&label_20.xrange);
    let symbol_1_yr = symbols[1].yrange();
    println!("Done!");
}

fn part1() {
    let (labels, symbols) = parse_map("input");
    println!("Got {} labels and {} symbols", labels.len(), symbols.len());

    let matches: Vec<u32> = labels.iter()
        .filter(|label| {
            symbols.iter().any(|symbol| {
                label.adjacent_to(symbol)
            })
        })
        .map(|label| label.number)
        .collect();

    let score: u32 = matches.iter()
        .sum();
    println!("{score}");
}

fn part2() {
    let (labels, symbols) = parse_map("input");
    let score: u32 = symbols
        .iter()
        .filter(|s| s.value == "*")
        .map(|symbol| {
            let labels: Vec<&Label> = labels.iter()
                .filter(|label| label.adjacent_to(symbol))
                .collect();
            labels
        })
        .filter(|labels| labels.len() == 2)
        .map(|labels| {
            let product: u32 = labels.iter()
                .map(|label| label.number)
                .product();
            product
        })
        .sum();
    println!("{score}");
}

fn parse_map(filename: &str) -> (Vec<Label>, Vec<Symbol>) {
    let file = fs::read_to_string(filename)
        .unwrap();

    let labels: Vec<Label> = file
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            NUMBER.find_iter(line)
                .map(move |m| Label {
                    number: m.as_str().parse().unwrap(),
                    y: y as i32,
                    xrange: Range {begin: m.start() as i32, end: (m.end() - 1) as i32 }
                })
        })
        .collect();

    let symbols: Vec<Symbol> = file
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            SYMBOL.find_iter(line)
                .map(move |m| Symbol {
                    value: String::from(m.as_str()),
                    y: y as i32,
                    x: m.start() as i32
                })
        })
        .collect();

    return (labels, symbols)
}
