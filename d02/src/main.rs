use std::cmp::max;
use std::fs;

struct Game {
    id: u32,
    pulls: Vec<Pull>
}

impl Game {
    fn minimal_set(&self) -> Pull {
        let mut minimal_pull = Pull {red: 0, green: 0, blue: 0};
        self.pulls.iter().for_each(|p| {
            minimal_pull.red = max(minimal_pull.red, p.red);
            minimal_pull.green = max(minimal_pull.green, p.green);
            minimal_pull.blue = max(minimal_pull.blue, p.blue);
        });

        minimal_pull
    }
}

struct Pull {
    red: u32,
    green: u32,
    blue: u32
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let games = read_games();

    let red_threshold = 12;
    let green_threshold = 13;
    let blue_threshold = 14;
    let score: u32 = games.iter()
        .filter(|g| g.pulls.iter().all(|p| {
            p.red <= red_threshold && p.green <= green_threshold && p.blue <= blue_threshold
        }))
        .map(|g| g.id)
        .sum();
    println!("{score}");
}

fn part2() {
    let games = read_games();
    let score: u32 = games.iter()
        .map(|g| g.minimal_set())
        .map(|p| p.red * p.green * p.blue)
        .sum();
    println!("{score}");

}

fn read_games() -> Vec<Game> {
    let games: Vec<Game> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(line_to_game)
        .collect();
    games
}

fn line_to_game(line: &str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();
    let game_id: u32 = parts[0]
        .split(" ")
        .last().unwrap()
        .parse().unwrap();
    let pulls: Vec<Pull> = parts[1]
        .split("; ")
        .map(parse_pull)
        .collect();

    return Game {
        id: game_id,
        pulls
    }
}

fn parse_pull(pull_str: &str) -> Pull {
    let mut pull = Pull {
        red: 0,
        green: 0,
        blue: 0
    };

    pull_str.split(", ")
        .for_each(|pair| {
            let parts: Vec<&str> = pair.split(" ").collect();
            let count: u32 = parts[0].parse().unwrap();
            match parts[1] {
                "red" => pull.red = count,
                "green" => pull.green = count,
                "blue" => pull.blue = count,
                _ => panic!("Got unexpected color: {}", parts[1])
            }
        });

    return pull;
}