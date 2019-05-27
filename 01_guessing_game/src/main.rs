use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number:");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("The secret is {}", secret);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
               .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
                          .expect("Please input a number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("YAY!"),
    }
}
