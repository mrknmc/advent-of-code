const SERIAL_NUMBER: usize = 5468;


fn main() {
    let mut max_area = 0;
    let mut max_idx = (0, 0, 0);
    let mut cells: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 301]; 301]; 301];
    for size in 1usize..=300 {
        for x in (1usize..=301-size).rev() {
            for y in (1usize..=301-size).rev() {
                let area = if size == 1 {
                    let rack_id = x + 10;
                    let power_level = rack_id * (rack_id * y + SERIAL_NUMBER);
                    let frac = power_level / 100;
                    let hund = if frac < 1 { 0 } else { frac % 10 };
                    hund as i64 - 5
                } else if size == 2 {
                    cells[x][y][1] + cells[x][y + 1][1] + cells[x + 1][y][1] + cells[x + 1][y + 1][1]
                } else {
                    let top_left = cells[x][y][size - 1];
                    let top_right = cells[x + 1][y][size - 1];
                    let bottom_left = cells[x][y + 1][size -1];
                    let bottom_right = cells[x + 1][y + 1][size - 1];

                    let middle = cells[x + 1][y + 1][size - 2];

                    let top_left_one = cells[x][y][1];
                    let top_right_one = cells[x + size - 1][y][1];
                    let bottom_left_one = cells[x][y + size - 1][1];
                    let bottom_right_one = cells[x + size - 1][y + size - 1][1];

                    (
                        top_left + top_right + bottom_left + bottom_right +
                        top_left_one + top_right_one + bottom_left_one + bottom_right_one -
                        (middle * 2)
                    ) / 2
                };
                cells[x][y][size] = area;
                if area > max_area {
                    max_area = area;
                    max_idx = (x, y, size);
                }
            }
        }
    }

    println!("max area: {:?}", max_area);
    println!("max idx: {:?}", max_idx);
}