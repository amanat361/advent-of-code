use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let file_path = env::args().nth(1).expect("Should have 1 argument");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // Only match exact mul(num,num) format
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    
    let mut sum = 0;

    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            let matched = cap.get(0).unwrap().as_str();
            let nums: Vec<i32> = matched[4..matched.len()-1]  // Remove "mul(" and ")"
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            
            let product = nums[0] * nums[1];
            sum += product;
        }
    }

    println!("Total sum of all multiplications: {}", sum);
}