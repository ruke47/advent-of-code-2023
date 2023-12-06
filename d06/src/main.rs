use std::time::SystemTime;

fn main() {
    part1();
    part2();
}

fn part1() {
    let times = vec![(41, 214), (96, 1789), (88, 1127), (94, 1055)];
    let mut score = 1;
    for (time, distance_record) in times {
        let wins = count_wins(time, distance_record);
        score *= wins;
    }
    println!("Part 1: {score}");
}

fn part2() {
    let time: i64 = 41968894;
    let distance_record: i64 = 214178911271055;

    let start = SystemTime::now();
    let wins = count_wins(time, distance_record);
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap().as_millis();
    println!("Part 2: {wins} in {duration} ms")
}


fn count_wins(time: i64, distance_record: i64) -> i32 {
    let mut wins = 0;
    for charge_time in 1..(time - 1) {
        let go_time = time - charge_time;
        let distance = charge_time * go_time;
        if distance > distance_record {
            wins += 1;
        }
    }
    wins
}
