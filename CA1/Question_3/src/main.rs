use std::io;
fn main() {
    loop {
        
    
    println!("Teccna's Bookshop");
    println!("-------------------------------------------");
    println!("Code  | Book Title              | Price");
    println!("-------------------------------------------");
    println!("R     | Rust for Beginners      | 15_000");
    println!("A     | AI Basics               | 12_500");
    println!("D     | Data Structures in rust | 20_000");
    println!("N     | Networking Essentialscar| 18_000");
    println!("-------------------------------------------");


        let mut input1 = String::new();
        let mut input2 = String::new();


    println!("\nEnter item code (R,A,D,N):");
 io::stdin().read_line(&mut input1).expect("Not a valid string");
 let code = input1.trim().to_uppercase();


    println!("\nEnter quantity:");
 io::stdin().read_line(&mut input2).expect("Not a valid string");
 let quantity:u32 = input2.trim().parse().expect("Enter an integer");

 let unit_price = match code.as_str() {
    "R" => 15_000.0,
    "A" => 12_500.0,
    "D" => 20_000.0,
    "N" => 18_000.0,
    _ => {println!("Invalid code!");
 continue;}
 };
 let mut total_cost = unit_price * quantity as f64;
 let discount = if quantity > 3 {
    total_cost * 0.10
 } else {
    0.0
 };
let final_amount = total_cost - discount;

println!("\n---PURCHASE SUMMARY---");
println!("Item code: {}", code);
println!("Quantity: {}", quantity);
println!("Unit price: {}", unit_price);
println!("Total cost: {}", total_cost);
if discount >0.0 {
    println!("Discount (10%): -{}", discount);
}
println!("Final Amount: {}", final_amount);

println!("\nAnother customer? (y/n): ");
let mut choice = String::new();
io::stdin().read_line(&mut choice).expect("Not a valid string");
let choice = choice.trim().to_lowercase();
if choice == "n" {
    println!("Come again!");
    break;
    }
}
}