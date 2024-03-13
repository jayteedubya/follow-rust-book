use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("+--------------------+\n| Guess the number! |\n+--------------------+\n");
    println!("enter your guess below:");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: i32 = guess.trim().parse().expect(
            "You lost the game before it even started. Try typing a number next time yeh varmint!",
        );

        println!("you guessed {guess}");
        println!("the secret number is: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, friendo"),
            Ordering::Equal => {
                println!("Dead nuts on, partner. You win!");
                break;
            }
            Ordering::Greater => println!("You overshot, bucko."),
        };
    }
}
