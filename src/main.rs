use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    //mut means mutable, we must specify this. Otherwise, we get an error.

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
