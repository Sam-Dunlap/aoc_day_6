use std::fs;

#[derive(Debug)]
struct Race {
    distance: u64,
    time: u64,
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("ok");
    let races = parse(&input);
    let mut product = 1;
    for race in races {
        product *= part_1(race)
    }
    println!("{product}");
    let big_race = parse_2(&input);
    println!("{}", part_1(big_race));
}

fn part_1(race: Race) -> u64 {
    let mut c = 0;
    for t in 0..=race.time {
        if t * (race.time - t) > race.distance {
            c += 1
        }
    }
    c
}

fn parse_2(input: &str) -> Race {
    let mut distance = 0;
    let mut time = 0;
    for line in input.lines() {
        if let Some(line) = line.strip_prefix("Time:") {
            let times: Vec<&str> = line.split_whitespace().collect();
            let mut concat = String::new();
            for time in times {
                concat.push_str(time);
            }
            time = concat.parse::<u64>().unwrap();
        } else {
            let ds: Vec<&str> = line
                .strip_prefix("Distance:")
                .unwrap()
                .split_whitespace()
                .collect();
            let mut concat = String::new();
            for d in ds {
                concat.push_str(d);
            }
            distance = concat.parse::<u64>().unwrap();
        }
    }
    Race { distance, time }
}

fn parse(input: &str) -> Vec<Race> {
    let mut distances: Vec<u64> = vec![];
    let mut times: Vec<u64> = vec![];
    for line in input.lines() {
        if let Some(line) = line.strip_prefix("Time:") {
            times = line
                .split_whitespace()
                .map(|time| time.parse().unwrap())
                .collect();
        } else {
            distances = line
                .strip_prefix("Distance:")
                .unwrap()
                .split_whitespace()
                .map(|d| d.parse().unwrap())
                .collect();
        }
    }
    let mut races: Vec<Race> = vec![];
    for i in 0..distances.len() {
        races.push(Race {
            distance: distances[i],
            time: times[i],
        })
    }
    races
}
