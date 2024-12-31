// Let's start with a simple struct representing a Person
struct Person {
    name: String,
    age: u32,
    skills: Vec<String>
}

fn main() {
    // First, let's create some test data
    let people = vec![
        Person {
            name: String::from("Alice"),
            age: 25,
            skills: vec![String::from("Rust"), String::from("Python")]
        },
        Person {
            name: String::from("Bob"),
            age: 30,
            skills: vec![String::from("JavaScript"), String::from("Rust")]
        },
        Person {
            name: String::from("Charlie"),
            age: 20,
            skills: vec![String::from("Python")]
        }
    ];

    // Exercise 1: Basic iterator usage
    // Find all people who know Rust
    let rust_devs: Vec<&Person> = people
        .iter()
        .filter(|person| person.skills.contains(&String::from("Rust")))
        .collect();
    
    println!("Rust developers: {}", rust_devs.len());
}