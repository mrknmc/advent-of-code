use std::env;
use std::fs::read_to_string;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    c_count: u32,
    m_count: u32,
    children: Vec<Node>,
    meta: Vec<u32>,
}

fn make_node(cs: &mut VecDeque<u32>, nodes: &mut VecDeque<Node>) -> Node {
    let c_count = cs.pop_front().unwrap();
    let m_count = cs.pop_front().unwrap();

    let mut children = Vec::new();
    for _ in 0..c_count {
        let n = make_node(cs, nodes);
        children.push(n);
    }

    let mut meta = Vec::new();
    for _ in 0..m_count {
        meta.push(cs.pop_front().unwrap());
    }
    return Node { m_count, c_count, children, meta };
}

fn read_file<'a>(filename: String) -> Node {
    let mut graph = VecDeque::new();
    let mut cs: VecDeque<u32> = read_to_string(filename).unwrap()
        .split(" ")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    make_node(&mut cs, &mut graph)
}

fn bfs_meta_sum(node: Node) -> u32 {
    let mut sum = 0;
    let mut q = VecDeque::new();
    q.push_back(node);
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        sum += n.meta.iter().fold(0, |acc, x| acc + x);
        for c in n.children {
            q.push_back(c);
        }
    }
    sum
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let root = read_file(filename);
    let sum = bfs_meta_sum(root);
    println!("{:#?}", sum);
}
