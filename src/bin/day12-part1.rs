use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn read_file<'a>(filename: String) -> (String, HashMap<String, String>) {
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    let mut lines = buf_reader.lines().map(|s| s.unwrap());
    let initial_state = lines.next().unwrap().split_off(15);
    lines.next();
    let rules = lines.map(|l| {
        let from_to = l.split(" => ").collect::<Vec<&str>>();
        (from_to[0].to_owned(), from_to[1].to_owned())
    }).collect();

    let mut prefix = String::from("....................................................................................................");
    let suffix = String::from("....................................................................................................");
    prefix.push_str(&initial_state);
    prefix.push_str(&suffix);

    (prefix, rules)
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let (mut state, rules) = read_file(filename);
    for i in 1..=20 {
        println!("Round: {}", i);
        println!("len: {}", state.len());
        let (mut begin, mut end) = (0, 5);
        let mut new_state = Vec::new();
        new_state.extend(vec![String::from("."), String::from(".")]);
        while end <= state.len() {
            let slice = &state[begin..end].to_owned();
            let replacement = rules[slice].to_owned();
            new_state.push(replacement);
            begin += 1;
            end += 1;
        }
        new_state.extend(vec![String::from("."), String::from(".")]);
        state = new_state.join("");
        println!("{}", state);
    }
    let mut plant_0 = 100;
    let positives = state[plant_0..].chars()
        .map(|s| if s == '#' { 1 } else { 0 })
        .zip(0..)
        .map(|(a, b)| a * b)
        .fold(0, |acc, x| acc + x);
    let negatives = state[..plant_0].chars().rev()
        .map(|s| if s == '#' { 1 } else { 0 })
        .zip((1..))
        .map(|(a, b)| - a * b)
        .fold(0, |acc, x| acc + x);
    let sum = positives + negatives;
    println!("sum: {}", sum);
}