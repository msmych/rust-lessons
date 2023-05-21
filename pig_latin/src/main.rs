use std::io;

fn main() {
    println!("Insert line to piggify:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let pig_latin = input
        .split_whitespace()
        .map(|w| to_pig_latin(w))
        .collect::<Vec<_>>()
        .join(" ");
    println!("Piggified version:");
    println!("{}", pig_latin);
}

fn to_pig_latin(s: &str) -> String {
    let s = s.to_lowercase();
    let first = s.chars().next();
    if let Some(c) = first {
        match c {
            'a' | 'e' | 'o' | 'u' => format!("{}-hay", s),
            _ => format!("{}-{}ay", &s[1..], c),
        }
    } else {
        s.to_string()
    }
}
