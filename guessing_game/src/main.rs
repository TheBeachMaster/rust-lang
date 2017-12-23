use std::io;
fn main() {
    println!("Hello, world!");
    println!("Guess The Number");
    println!("Enter the number");

    let mut guess =  String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to get input");
    println!("You guessed: {}", guess);
}
