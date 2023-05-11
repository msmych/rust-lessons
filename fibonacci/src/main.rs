use std::io;

fn main() {
    println!("Enter n:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    loop {
        let n: u32 = match n.trim().parse() {
            Ok(0) => continue,
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Fibonacci number #{n} is {}", fibonacci(n));
        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut first: u32 = 1;
    let mut second: u32 = 1;
    match n {
        1 => first,
        2 => second,
        _ => {
            let mut i = 2;
            while i < n {
                let temp = second;
                second += first;
                first = temp;
                i += 1;
            }
            second
        }
    }
}
