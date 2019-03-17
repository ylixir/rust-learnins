use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("You guessed: {}", num);
                num
            },
            Err(_) => {
                println!("{} is not a number", guess.trim());
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
