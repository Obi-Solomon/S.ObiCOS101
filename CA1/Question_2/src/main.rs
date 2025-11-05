use std::io;
fn main() {
    println!("Employee salary tax generator");
    loop{
        let mut input1 = String::new();
        let mut input2 = String::new();

    println!("\nEnter your full name:");
    io::stdin().read_line(&mut input1).expect("Not a valid string!");
    let name = input1.trim().to_uppercase();

    println!("\nEnter the amount of hours worked");
    io::stdin().read_line(&mut input2).expect("Not a valid string!");
    let hours:f32 = input2.trim().parse().expect("not a valid number");

    let rate:f32 = hours * 3000.0;
    if hours > 40.0 {let rate:f32 = hours * 4500.0;};

    println!("Your name is {}", name);
    println!("You have worked {} hours", hours);

    let gross_salary = rate;
     println!("Your Gross_ salary is: {}", gross_salary);
    if gross_salary > 100_000.0 {
      let net_pay = gross_salary - 2_000.0;
      println!("Your net pay is: {}", net_pay );
    }; 
    
    println!("\nAnother employee?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid string");
    let choice = choice.trim().to_lowercase();
    if choice == "n" {
        println!("Goodbye.");
        break;

    }



    }
}
