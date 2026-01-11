/*
 * Implementation: Rust (Gemini (Google AI) Translation from Python logic)
 * * Logic: "Bottom-Up Dynamic Programming" (Eat Level)
 * This algorithm solves the Maximum Path Sum problem by starting at the
 * second-to-last row and adding the maximum of the two available children
 * to each parent. It "eats" the levels until only the root contains the
 * total maximum path sum.
 */

struct Triangle {
    // We store the triangle as a 1D vector (e.g., [3, 7, 4, 2, 4, 6...])
    data: Vec<Vec<i32>>,
}

impl Triangle {
    // Initialize from your string input
    fn new(input: &str) -> Self {
        let data = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();
        Triangle { data }
    }

    // This is your "eat_level" logic
    fn eat_level(&mut self) -> bool {
        if self.data.len() <= 1 {
            return false;
        }

        // Get the index of the last row and the row above it
        let last_row_idx = self.data.len() - 1;

        // We clone the last row to look at its values safely
        let last_row = self.data.pop().unwrap();
        let current_row = self.data.last_mut().unwrap();

        for j in 0..current_row.len() {
            let left_val = last_row[j];
            let right_val = last_row[j + 1];

            // Add the max of the "children" to the parent
            current_row[j] += left_val.max(right_val);
        }

        true
    }

    // Custom display logic
    fn display(&self) {
        for row in &self.data {
            let formatted_row: Vec<String> = row.iter().map(|val| format!("{:04}", val)).collect();
            println!("{:?}", formatted_row); // Simplified for brevity
        }
    }
}

fn main() {
    // let input = "3\n7 4\n2 4 6\n8 5 9 3";
    let input = include_str!("triangle.txt");
    let mut t = Triangle::new(input);

    while t.eat_level() {
        t.display();
        println!("----------");
    }

    println!("Max Path Sum: {}", t.data[0][0]);
}
