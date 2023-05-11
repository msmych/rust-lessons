use std::io;

const FIVE: f64 = 5.0;
const NINE: f64 = 9.0;
const THIRTY_TWO: f64 = 32.0;

fn main() {
    println!("Choose Fahrenheit (f) or Celsius (c):");

    let mut system = String::new();

    io::stdin()
        .read_line(&mut system)
        .expect("Failed to read line");

    let system: char = match system.trim().parse() {
        Ok(c) => c,
        Err(_) => 'f',
    };

    println!(
        "Temperature in {}:",
        if system == 'f' {
            "Fahrenheit"
        } else {
            "Celsius"
        }
    );

    loop {
        let mut source_temp = String::new();

        io::stdin()
            .read_line(&mut source_temp)
            .expect("Failed to read line");

        let source_temp: f64 = match source_temp.trim().parse() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if system == 'f' {
            println!("Celsius: {}", (source_temp - THIRTY_TWO) * FIVE / NINE);
        } else {
            println!("Fahrenheit: {}", source_temp * NINE / FIVE + THIRTY_TWO);
        }
        break;
    }
}
