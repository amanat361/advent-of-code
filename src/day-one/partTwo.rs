use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Should have 1 argument");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Create two vectors to store our numbers
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // Process each line
    for line in contents.lines() {
        // Split the line by whitespace and collect into a vector of strings
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        // Parse first number and add to left array
        if let Ok(num) = numbers[0].parse::<i32>() {
            left_numbers.push(num);
        }
        
        // Parse second number and add to right array
        if let Ok(num) = numbers[1].parse::<i32>() {
            right_numbers.push(num);
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut left_ptr = 0;
    let mut right_ptr = 0;
    let mut similarity_score = 0;

    while left_ptr < left_numbers.len() && right_ptr < right_numbers.len() {
        // lets do the case where we keep going until we find a match
        while left_ptr < left_numbers.len() &&
              right_ptr < right_numbers.len() &&
              left_numbers[left_ptr] != right_numbers[right_ptr]
        {
            // the smaller value should increment
            if left_numbers[left_ptr] < right_numbers[right_ptr] {
                left_ptr += 1;
            }
            else {
                right_ptr += 1;
            }
        }

        // either we have a match or one of the two is at the end
        if left_ptr == left_numbers.len() || right_ptr == right_numbers.len() {
            break;
        }

        // now that we have a match, we keep going until we find a mismatch
        while left_ptr < left_numbers.len() &&
              right_ptr < right_numbers.len() &&
              left_numbers[left_ptr] == right_numbers[right_ptr]
        {
            similarity_score += right_numbers[right_ptr];
            right_ptr += 1;
        }
    }

    println!("The similarity score is: {}", similarity_score);

}