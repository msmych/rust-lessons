fn main() {
    println!("Hello, world!");
    let closure = returns_closure();
    println!("Closure returns {}", closure(1));
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
