use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Should have 1 argument");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = lines.len();
    let width = lines[0].len();
    
    let mut total_count = 0;
    
    fn check_sequence(chars: &[char]) -> bool {
        chars == &['X', 'M', 'A', 'S'] || chars == &['S', 'A', 'M', 'X']
    }

    // Check all directions from each starting position
    for row in 0..height {
        for col in 0..width {
            // Only proceed if we're on an X or S
            let current = lines[row][col];
            if current != 'X' && current != 'S' {
                continue;
            }

            let can_go_right = col <= width - 4;
            let can_go_down = row <= height - 4;
            let can_go_left = col >= 3;

            // Horizontal (right)
            if can_go_right {
                let sequence: Vec<char> = (0..4).map(|i| lines[row][col + i]).collect();
                if check_sequence(&sequence) {
                    total_count += 1;
                }
            }

            // Vertical (down)
            if can_go_down {
                let sequence: Vec<char> = (0..4).map(|i| lines[row + i][col]).collect();
                if check_sequence(&sequence) {
                    total_count += 1;
                }
            }

            // Diagonal down-right
            if can_go_right && can_go_down {
                let sequence: Vec<char> = (0..4).map(|i| lines[row + i][col + i]).collect();
                if check_sequence(&sequence) {
                    total_count += 1;
                }
            }

            // Diagonal down-left
            if can_go_left && can_go_down {
                let sequence: Vec<char> = (0..4).map(|i| lines[row + i][col - i]).collect();
                if check_sequence(&sequence) {
                    total_count += 1;
                }
            }
        }
    }

    println!("Total matches: {}", total_count);
}