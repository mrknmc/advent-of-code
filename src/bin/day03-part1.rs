extern crate regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
struct Patch {
    id: u64,
    x: u64,
    y: u64,
    w: u64,
    h: u64,
}

impl Patch {
    fn points(&self) -> Vec<(u64, u64)> {
        let mut vec = Vec::new();
        for p1 in self.x..self.x + self.w {
            for p2 in self.y..self.y + self.h {
                vec.push((p1, p1));
            }
        }
        vec
    }
}


fn read_file(filename: String) -> Vec<Patch> {
    let re = Regex::new(r"^#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)$").unwrap();
    let mut vec = Vec::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines().map(|s| s.unwrap()) {
        let c = re.captures(&line).expect("Line doesn't match regex");
        let p = Patch {
            id: c["id"].parse::<u64>().unwrap(),
            x: c["x"].parse::<u64>().unwrap(),
            y: c["y"].parse::<u64>().unwrap(),
            w: c["w"].parse::<u64>().unwrap(),
            h: c["h"].parse::<u64>().unwrap(),
        };
        vec.push(p);
    }
    vec
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let mut map: HashMap<(u64, u64), HashSet<u64>> = HashMap::new();
    let list = read_file(filename);
    for p in list {
        let id = p.id;
        for c in p.points() {
            map.entry(c).or_insert(HashSet::new()).insert(id);
        }
    }
    let overlaps = map.values().filter(|s| s.len() > 1).count();
    println!("{}", overlaps);
}
