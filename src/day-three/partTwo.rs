use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let file_path = env::args().nth(1).expect("Should have 1 argument");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // One regex to match all patterns we care about
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    
    let mut sum = 0;
    let mut enabled = true;

    for line in contents.lines() {
        // Find all matches and process them in order of appearance
        for cap in re.find_iter(line) {
            let matched = cap.as_str();
            
            match matched {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ if enabled && matched.starts_with("mul") => {
                    let nums: Vec<i32> = matched[4..matched.len()-1]
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect();
                    
                    let product = nums[0] * nums[1];
                    sum += product;
                }
                _ => {}
            }
        }
    }

    println!("Total sum of all multiplications: {}", sum);
}