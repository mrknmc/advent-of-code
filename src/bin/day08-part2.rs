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
    Node { m_count, c_count, children, meta }
}

fn read_file<'a>(filename: String) -> Node {
    let mut graph = VecDeque::new();
    let mut cs: VecDeque<u32> = read_to_string(filename).unwrap()
        .split(" ")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    make_node(&mut cs, &mut graph)
}

fn node_value(node: &Node) -> u32 {
    if node.c_count == 0 {
        return node.meta.iter().fold(0, |acc, x| acc + x);
    }
    node.meta.iter()
        .flat_map(|x| node.children.get(*x as usize - 1))
        .fold(0, |acc, c| acc + node_value(c))
}

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let root = read_file(filename);
    let sum = node_value(&root);
    println!("{:#?}", sum);
}
