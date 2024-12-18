use std::env;
use std::fs;

fn main() {
    // Read input file
    let file_path = env::args().nth(1).expect("Should have 1 argument");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut valid_count: i32 = 0;

    // Process each line in the file
    'line_loop: for line in contents.lines() {
        let mut prev = None;         // Previous number in sequence
        let mut prev_diff = None;    // Previous difference between numbers
        let mut has_problem: bool = false;

        // Check each number in the line
        for num in line.split_whitespace() {
            if let Ok(curr) = num.parse::<i32>() {
                if let Some(previous) = prev {
                    let curr_diff: i32 = curr - previous;

                    // Check if difference is within range (1-3)
                    if curr_diff.abs() < 1 || curr_diff.abs() > 3 {
                        if has_problem {
                            continue 'line_loop;
                        }
                        has_problem = true;
                    }

                    // Check if sequence direction remains consistent
                    if let Some(p_diff) = prev_diff {
                        if (curr_diff > 0) != (p_diff > 0) {
                            if has_problem {
                                continue 'line_loop;
                            }
                            has_problem = true;
                        }
                    }
                    prev_diff = Some(curr_diff);
                }
                prev = Some(curr);
            } else {
                println!("Warning: '{}' is not a valid number", num);
            }
        }
        
        valid_count += 1;  // Line passed all checks
    }

    println!("The number of valid lines is: {}", valid_count);
}