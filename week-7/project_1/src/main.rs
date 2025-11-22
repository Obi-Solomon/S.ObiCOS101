use std::io;

// A simple helper to read a number from the user
fn read_number(label: &str) -> f64 {
    println!("{}", label);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    println!("Welcome! Choose the calculation you want to perform:");
    println!("1 -> Area of Trapezium");
    println!("2 -> Area of Rhombus");
    println!("3 -> Area of Parallelogram");
    println!("4 -> Surface Area of Cube");
    println!("5 -> Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");

    let choice = choice.trim().parse().unwrap_or(0);

    match choice {
        1 => {
            println!("\n-- Trapezium Area --");
            let h = read_number("Enter the height:");
            let b1 = read_number("Enter base 1:");
            let b2 = read_number("Enter base 2:");
            let area = (h / 2.0) * (b1 + b2);
            println!("Area = {}", area);
        }

        2 => {
            println!("\n-- Rhombus Area --");
            let d1 = read_number("Enter diagonal 1:");
            let d2 = read_number("Enter diagonal 2:");
            println!("Area = {}", 0.5 * d1 * d2);
        }

        3 => {
            println!("\n-- Parallelogram Area --");
            let base = read_number("Enter the base:");
            let altitude = read_number("Enter the altitude:");
            println!("Area = {}", base * altitude);
        }

        4 => {
            println!("\n-- Cube Surface Area --");
            let side = read_number("Enter the length of one side:");
            println!("Surface Area = {}", 6.0 * side * side);
        }

        5 => {
            println!("\n-- Cylinder Volume --");
            let radius = read_number("Enter the radius:");
            let height = read_number("Enter the height:");
            let volume = std::f64::consts::PI * radius * radius * height;
            println!("Volume = {}", volume);
        }

        _ => println!("Invalid choice. Please run the program again."),
    }
}
