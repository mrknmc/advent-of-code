extern crate regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

const WORKERS: usize = 5;
const BASE_DELAY: u32 = 60;

#[derive(Debug)]
struct Edge {
    from: String,
    to: String,
}

fn find_roots(edges: &Vec<Edge>) -> HashSet<String> {
    let mut set2: HashSet<_> = edges.iter().map(|e| e.from.to_owned()).collect();
    for e in edges {
        set2.remove(&e.to);
    }
    set2
}


fn read_file<'a>(filename: String) -> Vec<Edge> {
    let mut vec = Vec::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    let re = Regex::new("Step (?P<from>[A-Z]) must be finished before step (?P<to>[A-Z]) can begin.").unwrap();
    for line in buf_reader.lines().map(|s| s.unwrap()) {
        let c = re.captures(&line).unwrap();
        vec.push(Edge { from: c["from"].to_owned(), to: c["to"].to_owned() });
    }
    vec
}

fn to_digit(s: &str) -> u32 {
    s.chars().next().unwrap().to_digit(36).unwrap() - 9
}

fn search(root_ids: Vec<String>, edge_map: HashMap<String, HashSet<String>>, prereqs: HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut ordering = Vec::new();
    let mut visited = HashSet::new();
    let mut expires: HashMap<u32, HashSet<String>> = HashMap::new();
    let mut heap = HashSet::new();
    let mut seconds = 0;
    let mut free = WORKERS;
    heap.extend(root_ids);
    loop {
        println!("--------");
        println!("Time: {}", seconds);
        println!("Free: {}", free);

        if let Some(expired) = expires.get(&seconds) {
            for e in expired {
                free += 1;
                println!("Expired: {}", e);
                visited.insert(e.to_string());
                ordering.push(e.to_string());
                if let Some(nodes) = edge_map.get(e) {
                    // not a leaf
                    let diff: Vec<String> = nodes.difference(&visited).cloned().collect();
                    heap.extend(diff);
                }
            }
        }

        println!("Free: {}", free);

        let mut candidates: Vec<String> = heap.iter().cloned()
            .filter(|h| !prereqs.contains_key(h) || prereqs[h].is_subset(&visited))
            .collect();
        candidates.sort_by(|a, b| a.cmp(&b));

        for c in candidates.iter().cloned().take(free) {
            free -= 1;
            let ttl = seconds + BASE_DELAY + to_digit(&c);
            heap.remove(&c);
            println!("Picked up: {}", c);
            let entry = expires.entry(ttl).or_insert(HashSet::new());
            entry.insert(c);
        }

        seconds += 1;
        println!("Free: {}", free);
        println!("--------");
        if heap.is_empty() && free == WORKERS {
            break;
        }
    }
    ordering
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let edges = read_file(filename);

    let mut root_ids: Vec<_> = find_roots(&edges).into_iter().collect();
    root_ids.sort_by(|a, b| a.cmp(&b));

    let mut edge_map: HashMap<String, HashSet<String>> = HashMap::new();
    for e in edges.iter().clone() {
        let entry = edge_map.entry(e.from.to_owned()).or_insert(HashSet::new());
        entry.insert(e.to.to_owned());
    }

    let mut prereqs: HashMap<String, HashSet<String>> = HashMap::new();
    for e in edges {
        let entry = prereqs.entry(e.to.to_owned()).or_insert(HashSet::new());
        entry.insert(e.from.to_owned());
    }

    let result = search(root_ids, edge_map, prereqs);
    println!("{:#?}", result.join(""));
}
