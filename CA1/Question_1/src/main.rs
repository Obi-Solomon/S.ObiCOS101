use std::io;
fn main() {
    println!("\nTemperature Reading Converter");

    let mut input = String::new();

    println!("Enter your Temperature (in celcius)");
    io::stdin().read_line(&mut input).expect("Not a valid string!");
    let c:f32 = input.trim().parse().expect("not a valid number");

    let f:f32 = (9.0 / 5.0)* c + 32.0;
    let k:f32 = c + 273.15;

 println!("Your temperature in Celcius, Farenheit and Kelvin are: {}, {}, {} ", c , f , k);

    if c <0.0 {
        println!("Freezing Point");
    }else if c > 0.0 && c <= 30.0 {
        println!("Normal Range");
    }else if c > 30.0 {
        println!("Hot Temperature");
    }else if c < -273.0 {
        println!("Invalid input");
    }
}