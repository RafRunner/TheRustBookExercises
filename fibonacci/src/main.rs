use std::io;

fn main() {
    loop {
        println!("Enter a value");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Error reading line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let sufix = match number {
            1 => "st",
            2 => "nd",
            _ => "th",
        };

        println!("The {}{} fibonacci number is: {}", number, sufix, fibonacci(number));
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 1;

    if n == 0 {
        return first;
    }

    if n == 1 {
        return second;
    }

    let mut val: u32 = second;

    for _ in 2..=n {
        val = first + second;
        let temp = second;
        second = val;
        first = temp;
    }

    return val;
}
