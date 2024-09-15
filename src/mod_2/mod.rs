use std::{cmp::Ordering, io};

use rand::Rng;

pub fn main() {
    // one();
    // two();
    three();
}

fn one() {
    println!("Guess the number");

    println!("Please input your guess:");

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("You guessed: {}", user_input)
}

fn two() {
    println!("Guess the numbwer");

    let random_number: i32 = rand::thread_rng().gen_range(1..10);

    println!("Random number is:{}", random_number);

    println!("Please input your guess:");

    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid Input");

    //Remember to add trim as it will take white spaces also and code will panic.
    let guess = guess.trim().parse::<i32>().expect("Enter a valid number");

    if random_number == guess {
        println!("You guessed the right value:{}", random_number)
    } else {
        println!("You entered wrong value");
    }
}

fn three() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1..10);

    println!("Random number is :{}", random_number);

    loop {
        println!("Please input number:");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Invalid Input");

        // This is called shadowing as we have already declared guess [rust support shadowing on same scope].
        let guess = match guess.trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                println!("Please enter valid number");
                continue;
            }
        };

        //Below we have used a match expression which we can use on any value which can be of certain types [enum].
        match guess.cmp(&random_number) {
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Congrats");
                break;
            }
            Ordering::Less => println!("Too small"),
        }
    }
}
