use std::time::SystemTime;

fn main() {
    part1();
    part2();
    part3();
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

fn part3() {
    let time: i64 = 41968894;
    let distance_record: i64 = 214178911271055;

    let wins = quadratic_your_wins(time, distance_record);
    assert_eq!(wins, 30077773);
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

fn quadratic_your_wins(time: i64, distance_record: i64) -> i32 {
    let ftime = time as f64;
    let fdistance_record = distance_record as f64;
    let lower: f64 = (ftime - (ftime.powf(2.0) - 4.0 * fdistance_record).sqrt())/2.0;
    let upper: f64 = (ftime + (ftime.powf(2.0) - 4.0 * fdistance_record).sqrt())/2.0;

    (upper - lower).ceil() as i32
}
