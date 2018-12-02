use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::LinkedList;


fn read_file(filename: String) -> LinkedList<HashMap<char, i64>> {
    let mut list = LinkedList::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines() {
        let mut map = HashMap::new();
        for c in line.unwrap().chars() {
            let counter = map.entry(c).or_insert(0);
            *counter += 1;
        }
        list.push_back(map);
    }
    list
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let list = read_file(filename);
    let mut doubles = 0;
    let mut triples = 0;
    for map in list {
        if map.values().any(|v| v == &2) {
            doubles += 1;
        }
        if map.values().any(|v| v == &3) {
            triples += 1;
        }
    }
    println!("{}", doubles * triples)
}
