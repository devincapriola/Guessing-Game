extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 100);
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail");
        let guess: u32 = guess.trim().parse().expect("Fail");
        if guess == secret_number {
            println!("You Win!");
            break;
        } else {
            if guess > secret_number {
                println!("Too High!");
            } else {
                println!("Too Low!");
            }
        }
    }
}
