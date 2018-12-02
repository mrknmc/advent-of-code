use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    let total = buf_reader.lines().fold(0, |_sum, l| l.unwrap().parse::<i64>().unwrap());
    println!("{}", total);
}
