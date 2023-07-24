fn main() {
    println!("Hello Rust!");

    let str = String::from("0Hello Rust");

    let char = str.chars().nth(0).unwrap();

    println!("char: {}", char.to_digit(10).unwrap());
}
