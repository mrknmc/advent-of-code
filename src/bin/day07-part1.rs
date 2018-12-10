extern crate regex;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


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

fn search(root_ids: Vec<String>, edge_map: HashMap<String, HashSet<String>>, prereqs: HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut ordering = Vec::new();
    let mut visited = HashSet::new();
    let mut heap = HashSet::new();
    heap.extend(root_ids);
    while !heap.is_empty() {
        let first = heap.iter().cloned()
            .filter(|h| !prereqs.contains_key(h) || prereqs[h].is_subset(&visited))
            .min_by(|a, b| a.cmp(&b))
            .unwrap();

        heap.remove(&first);
        visited.insert(first.to_owned());
        ordering.push(first.to_owned());
        if let Some(nodes) = edge_map.get(&first) {
            // not a leaf
            let diff: Vec<String> = nodes.difference(&visited).cloned().collect();
            heap.extend(diff);
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
