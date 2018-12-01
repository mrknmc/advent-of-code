use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::LinkedList;


fn read_file(filename: String) -> LinkedList<i64> {
    let mut list = LinkedList::new();
    let mut f = File::open(filename).expect("input file not found");
    let mut buf_reader = BufReader::new(f);
    for line in buf_reader.lines() {
        list.push_back(line.unwrap().parse::<i64>().unwrap());
    }
    list
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let list = read_file(filename);
    let mut seen = HashSet::new();
    let mut sum = 0;
    for num in list.iter().cycle() {
        let res = sum + num;
        if seen.contains(&res) {
            println!("{}", res);
            break;
        }
        sum = res;
        seen.insert(res);
    }
}
