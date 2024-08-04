use std::io;

fn main() {
  println!("Guess the number!");
  println!("Please input the number!");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to take input!");

  println!("You guessed {}",guess);
}