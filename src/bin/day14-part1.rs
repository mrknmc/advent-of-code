fn make_recipes(count: u32) -> Vec<usize> {
    let mut queue = vec![3, 7];
    let mut elf0 = 0;
    let mut elf1 = 1;
    for _ in 0..count - 2 {
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
    let queue = make_recipes(637071).into_iter().skip(637061).take(10).collect::<Vec<usize>>();
    println!("{:?}", queue);
}