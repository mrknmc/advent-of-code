extern crate regex;

use std::env;
use std::fs::File;
use std::io::{stdout, Write, BufRead, BufReader};
use std::collections::HashSet;
use std::{thread, time};
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Point {

    fn step(&self) -> Point {
        Point {
            x: self.x + self.vx,
            y: self.y + self.vy,
            vx: self.vx,
            vy: self.vy
        }
    }

    fn one_away(&self, other: &Point) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        (dx == 0 && dy == 1) || (dx == 1 && dy == 0) || (dx == 1 && dy == 1)
    }
}

fn read_file<'a>(filename: String) -> Vec<Point> {
    let mut vec = Vec::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    let re = Regex::new(r"^position=<\s?(?P<x>-?\d+), \s?(?P<y>-?\d+)> velocity=<\s?(?P<vx>-?\d+), \s?(?P<vy>-?\d+)>$").unwrap();
    for line in buf_reader.lines().map(|s| s.unwrap()) {
        let c = re.captures(&line).unwrap();
        vec.push(Point {
            x: c["x"].parse::<i64>().unwrap(),
            y: c["y"].parse::<i64>().unwrap(),
            vx: c["vx"].parse::<i64>().unwrap(),
            vy: c["vy"].parse::<i64>().unwrap(),
        });
    }
    vec
}

fn print_points(points: &Vec<Point>) {
    let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let point_set = points.iter().map(|p| (p.x, p.y)).collect::<HashSet<(i64, i64)>>();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if point_set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("{}", "-".repeat(max_x as usize));
    stdout().flush().unwrap();
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let mut points = read_file(filename);
    let mut i = 0;
    loop {
        let each_has_neighbor = points.iter()
            .all(|p1| points.iter()
                .any(|p2| p1.one_away(p2)));
        if each_has_neighbor {
            print_points(&points);
            println!("Time: {}", i);
            thread::sleep(time::Duration::from_secs(3));
        }
        points = points.iter().map(|p| p.step()).collect();
        i += 1;
    }
}