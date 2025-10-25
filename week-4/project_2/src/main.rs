use std::io;
fn main() {
    println!("\nEmployee Database Management System");

 let mut input1 = String::new();
 let mut input2 = String::new();

 println!("\nAre you experienced?");
 io::stdin().read_line(&mut input1).expect("Not a valid string");
 let experience = input1.trim().to_lowercase();

 println!("\nEnter your age:");
 io::stdin().read_line(&mut input2).expect("Not a valid string");
 let age:u32 = input2.trim().parse().expect("Enter an integer");

 let mut incentive:u32 = 0;

 if experience == "yes"{
    if age >=40 {
        incentive = 1_560_000;
    } else if age >= 30 && age < 40 {
        incentive = 1_480_000;
    } else if age < 28 {
        incentive = 1_300_000;
    } else if age >= 28 && age < 30 {
        incentive = 1_400_000
    }
 } else {
    incentive = 100_000;
 }
 println!("\nEmployee Incentive Report");
 println!("\nYour incentive: {}", incentive);
}