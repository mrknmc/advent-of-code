const SERIAL_NUMBER: usize = 5468;

fn main() {
    let mut max_square_sum = 0;
    let mut max_square_i = (298, 298);
    let mut cells: Vec<Vec<i64>> = vec![vec![0; 301]; 301];
    for x in (1usize..=300).rev() {
        for y in (1usize..=300).rev() {
            let rack_id = x + 10;
            let power_level = rack_id * ((rack_id * y) + SERIAL_NUMBER);
            let frac = power_level / 100;
            let hund = if frac < 1 { 0 } else { frac % 10 };
            cells[x][y] = hund as i64 - 5;
            if x <= 298 && y <= 298 {
                let square_sum = cells[x][y] + cells[x + 1][y] + cells[x + 2][y] +
                    cells[x][y + 1] + cells[x + 1][y + 1] + cells[x + 2][y + 1] +
                    cells[x][y + 2] + cells[x + 1][y + 2] + cells[x + 2][y + 2];
                if square_sum > max_square_sum {
                    max_square_sum = square_sum;
                    max_square_i = (x, y)
                }
            }
        }
    }
    println!("{:?}", max_square_i);
}