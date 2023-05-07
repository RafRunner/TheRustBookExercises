use rand::Rng;
use std::{io::{self, Write}, cmp::Ordering};

fn main() {
    loop {
        println!("Input a temperature:");
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Error in stdin");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("{} in C is {} in F", temp, celcius_to_fahrenheit(temp));
        println!("{} in F is {} in C", temp, fahrenheit_to_celcius(temp));
    }

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please, input your guess: ");
        io::stdout().flush().expect("Error flushing stdout");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading new line from stdin");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };
    }
}

fn celcius_to_fahrenheit(celc: f64) -> f64 {
    celc * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celcius(fahr: f64) -> f64 {
    (fahr - 32.0) * 5.0 / 9.0
}
