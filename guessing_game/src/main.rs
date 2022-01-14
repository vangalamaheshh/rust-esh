use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guessing Game!");
  let secret = rand::thread_rng().gen_range(1 .. 101);
  println!("Secret number: {}", secret);
  loop {
    println!("Enter a number: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error!");
    let guess: u32 = guess.trim().parse().expect("Please enter a number.");
    match guess.cmp(&secret) {
      Ordering::Less => println!("Too small."),
      Ordering::Greater => println!("Too big."),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
