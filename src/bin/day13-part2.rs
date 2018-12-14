use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::VecDeque;


fn read_file<'a>(filename: String) -> (VecDeque<char>, HashMap<(char, char, char, char, char), char>) {
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    let mut lines = buf_reader.lines().map(|s| s.unwrap());
    let initial_state = lines.next().unwrap().split_off(15).chars().collect();
    lines.next();
    let rules = lines.map(|l| {
        let from_to = l.split(" => ").collect::<Vec<&str>>();
        let from = from_to[0].chars().collect::<Vec<char>>();
        ((from[0], from[1], from[2], from[3], from[4]), from_to[1].chars().next().unwrap())
    }).collect();

    (initial_state, rules)
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let (mut state, rules) = read_file(filename);
    let mut new_state = VecDeque::new();
    for _ in 1..=92 {
        state.push_back('.');
        state.push_back('.');
        state.push_back('.');
        state.push_back('.');
        state.push_front('.');
        state.push_front('.');
        state.push_front('.');
        state.push_front('.');
        while state.len() >= 5 {
            let a = (state.pop_front().unwrap(), state.pop_front().unwrap(), state.pop_front().unwrap(), state.pop_front().unwrap(), state.pop_front().unwrap());
            let r = rules[&a];
            // println!("matching {:?} with {:?}", a, r);
            state.push_front(a.4);
            state.push_front(a.3);
            state.push_front(a.2);
            state.push_front(a.1);
            new_state.push_back(r);
        }
        state = new_state;
        println!("{}", state.iter().collect::<String>());
        new_state = VecDeque::new();
    }
    let plant_0 = (state.len() - 99) as i64 / 2;
    let mut sum = 0i64;
    let mut i = 0;
    while !state.is_empty() {
        let s = state.pop_front().unwrap();
        let val: i64 = if s == '#' { i as i64 - plant_0} else { 0 };
        sum += val;
        i += 1;
    }
    println!("sum: {}", sum);
}