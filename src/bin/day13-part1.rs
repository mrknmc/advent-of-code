use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

struct World {
    carts: Vec<Cart>,
    tracks: Vec<Vec<Track>>,
}

impl World { 

    fn has_crash(&self) -> bool {
        self.carts.iter().enumerate()
            .any(|(c1_id, c1)| self.carts.iter().enumerate()
                .any(|(c2_id, c2)| c1_id != c2_id && c1.crashed_with(&c2)))
    }

    fn crash_location(&self) -> Option<(usize, &Cart)> {
        self.carts.iter().enumerate()
            .find(|(c1_id, c1)| self.carts.iter().enumerate()
                .any(|(c2_id, c2)| c1_id != &c2_id && c1.crashed_with(&c2)))
    }

    fn tick(&mut self) {
        for mut cart in self.carts.as_mut_slice() {
            let track = &self.tracks[cart.x][cart.y];
            cart.tick(&track);
        }
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec = vec![vec![String::new(); 150]; 150];
        for (i, tracks) in self.tracks.iter().enumerate() {
            for (j, t) in tracks.iter().enumerate() {
                // transpose
                vec[j][i] = format!("{:?}", t);
            }
        }
        for c in &self.carts {
            // coords transposed at this point
            vec[c.y][c.x] = format!("{:?}", c.direction);
        }
        for row in vec {
            for col in row {
                write!(f, "{}", col);
            }
            writeln!(f, "");
        }
        writeln!(f, "")
    }
}

#[derive(Clone)]
enum Track {
    Empty,
    Horizontal,
    Vertical,
    Intersection,
    CurveForward,
    CurveBackward
}

impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Track::Empty => ' ',
            Track::Horizontal => '-',
            Track::Vertical => '|',
            Track::Intersection => '+',
            Track::CurveForward => '/',
            Track::CurveBackward => '\\',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
    Straight,
}

impl Turn {
    fn angle(&self) -> i32 {
        match self {
            Turn::Left => 90,
            Turn::Right => -90,
            Turn::Straight => 0
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        };
        write!(f, "{}", c)
    }
}

impl Direction {

    fn angle(&self) -> i32 {
        match self {
            Direction::North => 90,
            Direction::South => -90,
            Direction::West => 180,
            Direction::East => 0,
        }
    }

    fn from_angle(angle: i32) -> Direction {
        match angle {
            0 | 360 | -360 => Direction::East,
            90 | -270 => Direction::North,
            180 | -180 => Direction::West,
            270 | -90 => Direction::South,
            _ => { panic!("Unexpected number of degrees: {}", angle); }
        }
    }
    
    fn apply(&self, turn: &Turn) -> Direction {
        Self::from_angle(self.angle() + turn.angle() % 360)
    }
}

#[derive(Debug)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    turns: VecDeque<Turn>,
}

impl Cart {

    fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart { x, y, direction, turns: vec!(Turn::Left, Turn::Straight, Turn::Right).into_iter().collect() }
    }

    fn move_in_direction(&mut self, direction: &Direction) {
        let (x, y) = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        };
        self.x = (self.x as i32 + x) as usize;
        self.y = (self.y as i32 + y) as usize;
    }

    fn crashed_with(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y)
    }

    fn tick(&mut self, track: &Track) {
        let new_direction = match (&self.direction, track) {
            (_, Track::Empty) => { panic!("Cart not on track!"); }
            (Direction::South, Track::Horizontal) | (Direction::North, Track::Horizontal) => { panic!("Cart verical on horizontal track"); },
            (Direction::West, Track::Vertical) | (Direction::East, Track::Vertical) => { panic!("Cart horizontal on vertical track"); },
            (Direction::South, Track::CurveBackward) | (Direction::North, Track::CurveForward) => { Direction::East }
            (Direction::North, Track::CurveBackward) | (Direction::South, Track::CurveForward) => { Direction::West }
            (Direction::East, Track::CurveBackward) | (Direction::West, Track::CurveForward) => { Direction::South }
            (Direction::East, Track::CurveForward) | (Direction::West, Track::CurveBackward) => { Direction::North }
            (direction, Track::Vertical) | (direction, Track::Horizontal) => { direction.clone() }
            (direction, Track::Intersection) => {
                let turn = self.turns.pop_front().unwrap();
                let new_direction = direction.apply(&turn);
                self.turns.push_back(turn);
                new_direction
            }
        };
        self.move_in_direction(&new_direction);
        self.direction = new_direction;
    }
}

fn read_file<'a>(filename: String) -> World {
    let f = File::open(filename).expect("input file not found");
    let buf_reader = BufReader::new(f);
    let lines = buf_reader.lines().map(|s| s.unwrap());
    let mut carts = Vec::new();
    let mut tracks: Vec<Vec<Track>> = vec![vec![Track::Empty; 150]; 150];
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                ' ' => { continue; },
                '-' => { tracks[col][row] = Track::Horizontal; },
                '|' => { tracks[col][row] = Track::Vertical; }
                '/' => { tracks[col][row] = Track::CurveForward; },
                '\\' => { tracks[col][row] = Track::CurveBackward; },
                '+' => { tracks[col][row] = Track::Intersection; },
                'v' => { carts.push(Cart::new(col, row, Direction::South)); },
                '^' => { carts.push(Cart::new(col, row, Direction::North)); },
                '>' => { carts.push(Cart::new(col, row, Direction::East)); },
                '<' => { carts.push(Cart::new(col, row, Direction::West)); },
                c => { panic!("Unknown character: {}", c); }
            }
        }
    }

    // Add tracks to where the carts are
    for cart in &carts {
        tracks[cart.x][cart.y] = match (
            &tracks[cart.x - 1][cart.y],
            &tracks[cart.x + 1][cart.y],
            &tracks[cart.x][cart.y - 1],
            &tracks[cart.x][cart.y + 1]
        ) {
            // left, right, top, bottom
            (Track::Horizontal, Track::Horizontal, Track::Vertical, Track::Vertical) => Track::Intersection,
            (Track::Horizontal, Track::Horizontal, _, _) => Track::Horizontal,
            (_, _, Track::Vertical, Track::Vertical) => Track::Vertical,
            (Track::Horizontal, _, Track::Vertical, _) => Track::CurveForward,
            (Track::Horizontal, _, _, Track::Vertical) => Track::CurveBackward,
            (_, Track::Horizontal, Track::Vertical, _) => Track::CurveBackward,
            (_, Track::Horizontal, _, Track::Vertical) => Track::CurveForward,
            (Track::Intersection, Track::Intersection, Track::Intersection, Track::Intersection) => Track::Vertical,
            (Track::Intersection, Track::Intersection, _, _) => Track::Horizontal,
            (Track::Intersection, Track::Horizontal, _, _) => Track::Horizontal,
            (Track::Horizontal, Track::Intersection, _, _) => Track::Horizontal,
            (_, _, Track::Intersection, Track::Intersection) => Track::Vertical,
            (_, _, Track::Intersection, Track::Vertical) => Track::Vertical,
            (_, _, Track::Vertical, Track::Intersection) => Track::Vertical,
            pattern => { panic!("Unrecognized pattern around cart: {:?} on coords: {}, {}", pattern, cart.x, cart.y); }
        };
    }
    World { carts, tracks, }
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let mut world = read_file(filename);
    while !world.has_crash() {
        // println!("{:?}", world);
        world.tick();
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("{:?}", world);
    println!("{:?}", world.crash_location());
}