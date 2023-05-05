use std::io;

mod vectors;
mod pig_latin;
mod employees;

fn main() {
    let vector = vec![23, 4, 1, 98, 32, 1, -32, 6, -293];

    let median = vectors::median(&vector);
    let mode = vectors::mode(&vector);

    println!("{:?}", median);
    println!("{:?}", mode);

    println!("{}", pig_latin::pigfy("To be or not to be, that is the question"));

    let mut company = employees::Company::new();

    loop {
        println!("Enter a command");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error reading line");

        company.execute_command(&command);
        println!();
    }
}
