extern crate rand;


use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    println!("Enter the number");

    let sec_num = rand::thread_rng().gen_range(1, 101);

    let mut guess =  String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to get input");
    println!("You guessed: {}", guess);
    println!("The Secret Number was : {}", sec_num);
}
