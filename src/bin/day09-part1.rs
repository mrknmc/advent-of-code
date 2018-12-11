use std::collections::VecDeque;

fn main() {
    let (players, last_value) = (459, 72103);
    let mut scores = vec![0; players as usize];
    let mut list = VecDeque::new();
    list.push_back(0);
    let mut cur: i32 = 0;
    for (val, p) in (1..=last_value).into_iter().zip((0..players).cycle()) {
        let list_len = list.len() as i32;
        cur = if val % 23 == 0 {
            let to_remove = if cur >= 7 { cur - 7 } else { list_len + (cur - 7 % list_len) };
            let removed = list.remove(to_remove as usize).unwrap();
            scores[p as usize] += val + removed;
            to_remove
        } else {
            let mut next = (cur + 2) % list_len;
            next = if next == 0 { list_len } else { next };
            list.insert(next as usize, val);
            next
        }
        // println!("round: {:?} cur: {:?} cur_val: {:?} list_len: {:?} list: {:?}", val, cur, list[cur as usize], list.len(), list);
    }
    let max_score = scores.iter().max_by(|a, b| a.cmp(&b)).unwrap();
    println!("max_score: {:?}", max_score);
}
