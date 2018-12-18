
fn make_recipes(pattern: Vec<usize>) -> Vec<usize> {
    let mut queue = vec![3, 7];
    let mut elf0 = 0;
    let mut elf1 = 1;
    loop {
        let found_pattern = {
            let mut last_6 = queue.iter().rev().take(7).map(|x| *x).collect::<Vec<usize>>();
            last_6.reverse();
            last_6.windows(pattern.len()).any(|a| a == pattern.as_slice())
        };
        if found_pattern {
            break;
        }
        let recipe0 = queue[elf0];
        let recipe1 = queue[elf1];
        let sum = recipe0 + recipe1;
        if sum > 9 {
            let last_digit = sum % 10;
            let first_digit = sum / 10;
            queue.push(first_digit);
            queue.push(last_digit);
        } else {
            queue.push(sum);
        }
        let recipe_count = queue.len();
        elf0 = (elf0 + 1 + recipe0) % recipe_count;
        elf1 = (elf1 + 1 + recipe1) % recipe_count;
    }
    queue
}


fn main() {
    let queue = make_recipes(vec![6, 3, 7, 0, 6, 1]);
    println!("{:?}", queue.len());
}