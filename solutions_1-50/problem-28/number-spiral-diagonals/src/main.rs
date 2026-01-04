const INPUT_SIZE: usize = 1001;
const SIZE: usize = INPUT_SIZE + 2; // This unfortunately needs to be question number + 2. For this problem, 1001 -> 1003 (fills with 0 layer).
const DEBUG: bool = false;
fn main() {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; SIZE]; SIZE];
    grid = fill_diags(grid);
    if DEBUG {
        display(grid.clone());
    }
    println!("sum(diags({}x{})) = {}", INPUT_SIZE, INPUT_SIZE, sum(grid));
}

fn fill_diags(mut grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // Set the center point. (1x1 square)
    grid[SIZE / 2][SIZE / 2] = 1;
    let mut p: usize = 1;
    let mut c: u32 = 2;
    for i in 0..SIZE / 2 {
        println!("Square size = 2*{}+1 = {}", p, 2 * p + 1);
        // each new layer will be at grid[SIZE / 2 + p][SIZE / 2 - p + 1] -- flip indices
        grid[SIZE / 2 - p + 1][SIZE / 2 + p - 1] = c - 1; // Top right diagonal
        if i < SIZE / 2 - 1 {
            // Bottom right diagonal
            // grid[SIZE / 2 - p + 1 + (2 * p - 1)][SIZE / 2 + p] = c + 2 * p as u32 - 1;
            // Clean up algebra:
            grid[SIZE / 2 + p][SIZE / 2 + p] = c + 2 * p as u32 - 1;
            // Bottom left diagonal
            // grid[SIZE / 2 - p + 1 + (2 * p - 1)][SIZE / 2 + p - 2 * p] = c + 2 * p as u32 - 1 + 2 * p as u32;
            // Clean up algebra:
            grid[SIZE / 2 + p][SIZE / 2 - p] = c - 1 + 4 * p as u32;
            // Top left diagonal
            // grid[SIZE / 2 - p + 1 + (2 * p - 1) - 2 * p][SIZE / 2 + p - 2 * p] = c + 2 * p as u32 - 1 + 2 * p as u32 + 2 * p as u32;
            grid[SIZE / 2 - p][SIZE / 2 - p] = c - 1 + 6 * p as u32;
        }
        p += 1;
        c += 8 * (p - 1) as u32;
    }
    return grid;
}

fn display(grid: Vec<Vec<u32>>) {
    for row in &grid {
        for value in row {
            print!("{:4} ", value);
        }
        println!();
    }
}

fn sum(grid: Vec<Vec<u32>>) -> u128 {
    let mut s: u128 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            s += grid[i][j] as u128;
        }
    }
    return s;
}
