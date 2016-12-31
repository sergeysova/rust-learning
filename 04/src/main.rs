extern crate rand;

use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please, enter your assumption.");

        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("Can't read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your try: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("So less!"),
            Ordering::Greater => println!("So big!"),
            Ordering::Equal => { println!("You win!"); break; },
        }
    }
}
