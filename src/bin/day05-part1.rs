use std::env;
use std::fs::read_to_string;
use std::collections::LinkedList;

fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let string = read_to_string(filename).unwrap();

    let mut chars = LinkedList::new();
    chars.extend(string.chars());
    chars.pop_back(); // newline

    let mut list: LinkedList<char> = LinkedList::new();

    for c in chars {
        if let Some(last) = list.pop_back() {
            // have one on stack
            if last == c || last.to_lowercase().ne(c.to_lowercase()) {
                // merge
                list.push_back(last);
                list.push_back(c);
            }
        } else {
            // nothing on stack
            list.push_back(c);
        }
    }
    println!("{:?}", list.len());
}
