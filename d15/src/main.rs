use std::fs;
use crate::Instruction::{Add, Remove};

struct Hash {
    val: u32
}

impl Hash {
    fn new() -> Hash {
        Hash { val: 0 }
    }

    fn of_word(word: &str) -> Hash {
        let mut h = Hash::new();
        h.hash_word(word);
        h
    }

    fn hash_word(&mut self, word: &str) {
        word.chars()
            .for_each(|ch| self.hash_ch(ch));
    }

    fn hash_ch(&mut self, ch: char) {
        self.val += ch as u32;
        self.val *= 17;
        self.val = self.val % 256;
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Instruction {
    Add(String, u32),
    Remove(String)
}

impl Instruction {
    fn from_string(word: &str) -> Instruction {
        let mut tag = String::new();
        let mut iter = word.chars();
        loop {
            let ch = iter.next().unwrap();
            if ch.is_alphabetic() {
                tag.push(ch);
            } else if ch == '-' {
                return Remove(tag);
            } else if ch == '=' {
                let val = iter.next().unwrap().to_digit(10).unwrap();
                return Add(tag, val);
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Lens {
    tag: String,
    val: u32,
}

impl Lens {
    fn new(tag: &str, val: u32) -> Lens {
        Lens {tag: String::from(tag), val }
    }
}


fn main() {
    part1();
    part2();
}

fn part1() {
    let sum: u32 = read_steps("input").iter()
        .map(|word| {
            let h = Hash::of_word(word);
            // println!("{}: {}", word, h.val);
            h.val
        })
        .sum();
    println!("Part 1: {sum}");
}

fn part2() {
    let instructions: Vec<Instruction> = read_steps("input")
        .iter()
        .map(|word| Instruction::from_string(word))
        .collect();
    let mut bins : Vec<Vec<Lens>>= vec![vec![]; 256];
    for instr in instructions {
        match instr {
            Add(tag, val) => {
                let bin_id = Hash::of_word(&tag).val as usize;
                let bin = &mut bins[bin_id];
                let slot_idx = bin.iter()
                    .enumerate()
                    .filter(|(_, s)| s.tag == tag)
                    .map(|(idx, _)| idx)
                    .next();
                let new_lens = Lens::new(&tag, val);
                match slot_idx {
                    Some(idx) => bin[idx] = new_lens,
                    None =>  bin.push(new_lens)
                }
            },
            Remove(tag) => {
                let bin_id = Hash::of_word(&tag).val as usize;
                let bin  = &mut bins[bin_id];
                let remove_idx = bin.iter()
                    .enumerate()
                    .filter(|(_, s)| s.tag == tag)
                    .map(|(idx, _)| idx)
                    .next();
                if let Some(idx) = remove_idx {
                    bin.remove(idx);
                }
            }
        }
    }

    let sum: usize = bins.iter().enumerate()
        .flat_map(|(id, bin)| {
            if !bin.is_empty() {
                // println!("Bin {id}: {:?}", bin);
            }
            bin.iter().enumerate()
                .map(move |(slot, lens)| {
                    (id + 1) * (slot + 1) * (lens.val as usize)
                })
        })
        .sum();

    println!("Part 2: {sum}");
}

fn read_steps(filename: &str) -> Vec<String> {
    fs::read_to_string(filename).unwrap()
        .trim_end()
        .split(",")
        .map(String::from)
        .collect()
}
