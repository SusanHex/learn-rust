// Rusts standard input/output
use std::io;


fn main() {
    println!("Guess the number!");
    println!("What is your guess?: ");
    // create a mutable String variable
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("Your guess was: {}", guess)

}
