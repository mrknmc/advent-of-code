extern crate regex;
extern crate chrono;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;
use chrono::{NaiveDateTime, Timelike, Duration};

#[derive(Debug)]
struct Event {
    dt: NaiveDateTime,
    et: EventType
}


#[derive(Debug)]
enum EventType {
    FallsAsleep,
    WakesUp,
    BeginsShift(u64),
}


fn read_file(filename: String) -> Vec<Event> {
    let re = Regex::new(r"^\[(?P<dt>.+)\] (?P<type>(wakes up|falls asleep|Guard #(?P<g>\d+) begins shift))$").unwrap();
    let mut vec = Vec::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines().map(|s| s.unwrap()) {
        let c = re.captures(&line).expect("Line doesn't match regex");
        let dt = NaiveDateTime::parse_from_str(&c["dt"], "%Y-%m-%d %H:%M").unwrap();
        let e = match c.name("type") {
            Some(m) if m.as_str() == "wakes up" => Event { dt, et: EventType::WakesUp },
            Some(m) if m.as_str() == "falls asleep" => Event { dt, et: EventType::FallsAsleep },
            Some(_) => Event { dt, et: EventType::BeginsShift(c["g"].parse::<u64>().unwrap()) },
            None => panic!("Can't match line against events"),
        };
        vec.push(e);
    }
    vec
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let mut events = read_file(filename);
    events.sort_by(|a, b| a.dt.cmp(&b.dt));

    let mut guard_map = HashMap::new();
    let mut minute_map = HashMap::new();

    let mut guard = None;
    let mut mins = Vec::new();
    let mut total = Duration::zero();

    let mut sleep_start = None;
    for e in events {
        match (e, guard) {
            (Event { et: EventType::BeginsShift(new_guard), .. }, None) => {
                // first guard
                guard = Some(new_guard);
            },
            (Event { et: EventType::BeginsShift(new_guard), .. }, Some(g)) => {
                let duration = guard_map.entry(g).or_insert(Duration::zero());
                *duration = *duration + total;

                let dist = minute_map.entry(g).or_insert(Vec::new());
                dist.extend(mins);

                // Reset
                mins = Vec::new();
                total = Duration::zero();
                guard = Some(new_guard);
            },
            (Event { et: EventType::WakesUp, dt }, _) => {
                let start_dt: NaiveDateTime = sleep_start.unwrap();
                total = total + (dt - start_dt);

                for i in start_dt.minute()..dt.minute() {
                    mins.push(i);
                }
            },
            (Event { et: EventType::FallsAsleep, dt }, _) => {
                sleep_start = Some(dt);
            }
        };
    }
    let mut winner_g = None;
    let mut winner_m = None;
    let mut max = 0;
    for (g, mins) in minute_map.iter() {
        let mut counts = HashMap::new();
        for m in mins {
            let count = counts.entry(m).or_insert(0);
            *count += 1;
        }
        if let Some((&minute, &count)) = counts.iter().max_by(|a, b| a.1.cmp(b.1)) {
            if count >= max {
                winner_g = Some(g);
                winner_m = Some(minute);
                max = count;
            }
        }
    }
    println!("{:?}", winner_g);
    println!("{:?}", winner_m);
}
