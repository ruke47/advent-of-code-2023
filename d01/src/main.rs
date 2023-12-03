use std::fs;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum: u32 = fs::read_to_string("input")
        .expect("Couldn't read file")
        .lines()
        .map(|line| line_to_int(line, first_digit_1))
        .sum();

    println!("{sum}");
}

fn first_digit_1(line: &str) -> &str {
    let first_num_re = Regex::new(r"\d").unwrap();
    let digit_str = first_num_re
        .find_iter(line)
        .next()
        .map(|d| d.as_str())
        .unwrap();

    return digit_str;
}

fn line_to_int(line: &str, digit_finder: fn(&str) -> &str) -> u32 {
    let digits_1 = digit_finder(line).to_string();
    let reverse_line: String = line.to_string().chars().rev().collect();
    let digits_2 = digits_1 + digit_finder(&reverse_line);

    return digits_2.parse().unwrap();
}

fn part2() {
    let sum:u32 = fs::read_to_string("input")
        .expect("Couldn't read file")
        .lines()
        .map(|line| line_to_int_2(line))
        .sum();
    println!("{sum}");
}

fn line_to_int_2(line: &str) -> u32 {
    let num_re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)")
        .unwrap();
    let first_digit = num_re
        .find_iter(line)
        .next()
        .map(|d| d.as_str())
        .map(parse_digit)
        .unwrap()
        .to_string();
    let backwards_num_re = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")
        .unwrap();
    let last_unparsed_digit = backwards_num_re
        .find_iter(reverse(line).as_str())
        .next()
        .map(|m| m.as_str())
        .map(reverse)
        .unwrap();
    let last_digit = parse_digit(last_unparsed_digit.as_str());


    let combo = first_digit.clone() + last_digit;
    let two_digit = combo.parse().unwrap();
    // println!("{line}: {first_digit} {last_digit} => {combo} ({two_digit})");
    return two_digit
}

fn parse_digit(d: &str) -> &str {
    match d {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => d
    }
}

fn reverse(word: &str) -> String {
    return word.to_string().chars().rev().collect();
}