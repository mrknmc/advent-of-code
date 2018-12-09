use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    id: usize,
}

impl Point {
    fn dist(&self, p2: &Point) -> i64 {
        (&self.x - p2.x).abs() + (&self.y - p2.y).abs()
    }

    fn dist_all(&self, points: &Vec<Point>) -> i64 {
        points.iter().fold(0, |acc, p| acc + self.dist(p))
    }
}


fn read_file(filename: String) -> Vec<Point> {
    let mut vec = Vec::new();
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    for (id, line) in buf_reader.lines().map(|s| s.unwrap()).enumerate() {
        let split = line.split(", ")
            .map(|p| p.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let p = Point { id, x: split[0], y: split[1] };
        vec.push(p);
    }
    vec
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let points = read_file(filename);
    let min_x = points.iter().min_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let min_y = points.iter().min_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    let max_x = points.iter().max_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let max_y = points.iter().max_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    let mut area = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = Point { id: 0, x, y };
            let dist_sum = p.dist_all(&points);
            if dist_sum < 10000 {
                area += 1;
            }
        }
    }
    println!("{:?}", area);
}
