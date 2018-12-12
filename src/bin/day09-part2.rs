use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    val: u32,
}

impl Node {

    fn c_cwise(&self, steps: usize) -> Rc<RefCell<Node>> {
        let mut p = Rc::clone(self.prev.as_ref().unwrap());
        for _ in 0..steps - 1 {
            p = {
                let b = p.borrow();
                Rc::clone(b.prev.as_ref().unwrap())
            };
        }
        p
    }

    fn cwise(&self, steps: usize) -> Rc<RefCell<Node>> {
        let mut p = Rc::clone(self.next.as_ref().unwrap());
        for _ in 0..steps - 1 {
            p = {
                let b = p.borrow();
                Rc::clone(b.next.as_ref().unwrap())
            };
        }
        p
    }
}

fn main() {
    let (players, last_value) = (459, 7210300);
    let mut scores = vec![0; players as usize];
    let mut cur = Rc::new(RefCell::new(Node { val: 0, prev: None, next: None }));
    (*cur.borrow_mut()).next = Some(Rc::clone(&cur));
    (*cur.borrow_mut()).prev = Some(Rc::clone(&cur));
    for (val, p) in (1..=last_value).into_iter().zip((0..players).cycle()) {
        cur = if val % 23 == 0 {
            let node = cur.borrow().c_cwise(7);
            scores[p as usize] += val + node.borrow().val;
            let next = cur.borrow().c_cwise(6);
            let prev = cur.borrow().c_cwise(8);
            prev.borrow_mut().next = Some(Rc::clone(&next));
            next.borrow_mut().prev = Some(Rc::clone(&prev));
            next
        } else {
            let prev_node = cur.borrow().cwise(1);
            let next_node = cur.borrow().cwise(2);
            let new_ref = Rc::new(RefCell::new(Node { val, prev: Some(Rc::clone(&prev_node)), next: Some(Rc::clone(&next_node))}));
            prev_node.borrow_mut().next = Some(Rc::clone(&new_ref));
            next_node.borrow_mut().prev = Some(Rc::clone(&new_ref));
            new_ref
        };
        // println!("round: {:?} cur_val: {:?}", val, cur.borrow().val);
    }
    let max_score = scores.iter().max_by(|a, b| a.cmp(&b)).unwrap();
    println!("max_score: {:?}", max_score);
}
