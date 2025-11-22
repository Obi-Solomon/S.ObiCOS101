fn main() {
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}",datatype_tuple );

    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}",no_datatype_tuple );

    println!("Value at Index 0 = {}",datatype_tuple.0 );
    println!("Value at index 1 = {}",datatype_tuple.1 );
    println!("Value at input 2 = {}",datatype_tuple.2 );
}