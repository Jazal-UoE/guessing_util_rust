use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess. Exit command available");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        if guess.trim() == "exit" {
            print!("exiting software");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {} ", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
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
