use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read linee\n");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You have entered: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
