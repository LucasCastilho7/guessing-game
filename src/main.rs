use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
  println!("Insert a number: ");

  let secret_number = rand::thread_rng().gen_range(1..11);
  
  loop {
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
      
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please, type a number.");
        continue;
      }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small :("),
      Ordering::Greater => println!("Too large :("),
      Ordering::Equal => {
        println!("You've guessed right!! :)");
        break;
      },
    }
  }
}
