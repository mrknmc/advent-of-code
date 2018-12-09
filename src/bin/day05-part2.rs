use std::env;
use std::fs::read_to_string;
use std::collections::LinkedList;
use std::collections::HashSet;


fn reduce(polymer: String) -> String {
    let mut chars = LinkedList::new();
    chars.extend(polymer.chars());
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

    list.into_iter().collect()
}


fn main() {
    let filename = env::args().nth(1).expect("No argument filename passed");
    let string = read_to_string(filename).unwrap();

    let mut char_set = HashSet::new();
    char_set.extend(string.chars());

    let mut min = usize::max_value();

    for c in char_set {
        let s = string.replace(c.to_lowercase().next().unwrap(), "").replace(c.to_uppercase().next().unwrap(), "");
        let r = reduce(s).len();
        if r <= min {
            min = r;
        }
    }
    println!("{:?}", min);
}
