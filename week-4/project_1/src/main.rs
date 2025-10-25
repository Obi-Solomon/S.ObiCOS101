use std::io;

fn main() {
    println!("\nFind the roots of the equation");

let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

println!("\nEnter the value of a:");
io::stdin().read_line(&mut input1).expect("Not a valid String");
let a:f32 = input1.trim().parse().expect("Please enter an integer");

println!("\nEnter the value of b:");
io::stdin().read_line(&mut input2).expect("Not a valid String");
let b:f32 = input2.trim().parse().expect("Please enter an integer");

println!("\nEnter the value of c:");
io::stdin().read_line(&mut input3).expect("Not a valid String");
let c:f32 = input3.trim().parse().expect("Please enter an integer");


let discriminant = b.powf(2.0) - (4.0*a*c);
println!("\nThe discriminant is: {}", discriminant);

if discriminant > 0.0 {
    let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2 = (-b + discriminant.sqrt()) / (2.0 * a);
    println!("Your equation has two distinct real roots: x1 = {}, x2 = {}", root1, root2);
   
} else if discriminant == 0.0 {
    let root = -b / (2.0 * a);
    println!("Your equation has one real root : x = {}", root);

} else {
    println!("Your equation has no real roots");
}
}
