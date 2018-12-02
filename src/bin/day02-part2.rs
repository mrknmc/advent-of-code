use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;


fn read_file(filename: String) -> Vec<Vec<String>> {
    let mut vec = Vec::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines().map(|s| s.unwrap()) {
        let mut inner_vec = Vec::new();
        for i in 0..line.len() {
            let mut s = line.clone();
            s.remove(i);
            inner_vec.push(s);
        }
        vec.push(inner_vec);
    }
    vec
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let list = read_file(filename);
    let mut seen = HashSet::new();
    for (i, corrections) in list.iter().enumerate() {
        for c in corrections {
            if seen.contains(c) {
                println!("{}", c);
                break;
            }
        }
        seen.extend(corrections);
    }
}
