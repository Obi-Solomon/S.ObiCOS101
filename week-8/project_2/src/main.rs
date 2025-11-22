use std::io;

#[derive(Debug)]
struct Developer {
    name: String,
    experience: u32,
}

fn ask(question: &str) -> String {
    println!("{}", question);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Input error");
    response.trim().to_string()
}

fn main() {
    println!("--- EY Nigeria: Highest Experience Finder ---");

    let mut candidates: Vec<Developer> = Vec::new();

    let total = ask("How many developers are being interviewed?")
        .parse::<usize>()
        .expect("Please enter a valid number");

    for i in 1..=total {
        println!("\nDeveloper {}", i);

        let name = ask("Enter developer's name:");
        let experience = ask("Enter their years of programming experience:")
            .parse::<u32>()
            .expect("Experience must be a number");

        candidates.push(Developer { name, experience });
    }

    let most_experienced = candidates
        .iter()
        .max_by_key(|dev| dev.experience)
        .expect("No candidates available");

    println!("\n===================================");
    println!("Most Experienced Developer:");
    println!("Name: {}", most_experienced.name);
    println!("Experience: {} years", most_experienced.experience);
    println!("===================================");
}
