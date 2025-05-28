use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number: i32 = rand::rng().random_range(1..=100);

    println!("The secret number is {}", secret_number);
    println!("Please input your guses.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed {} ", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    // priotln!("{result}")
}

// match io::stdin().read_line(&mut guess) {
//     Ok(_) => {
//         println!("You entered: {guess}")
//     }
//     Err(error) => {
//         panic!("Failed to read line: {}", error);
//     }
// }
// println!("you double entered {guess}")
