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

    let mut sum: i32 = 0;

    // for loop to compare the two vectors
    for i in 0..left_numbers.len() {
        if left_numbers[i] > right_numbers[i] {
            sum += left_numbers[i] - right_numbers[i];
        }
        else {
            sum += right_numbers[i] - left_numbers[i];
        }
    }

    println!("The sum is: {}", sum);

}