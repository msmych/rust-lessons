use add_one;
use add_two;
use rand;

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}", add_one::add_one(num));
    println!("Hello, world! {num} plus two is {}", add_two::add_two(num));

    let answer = do_twice(add_one::add_one, 5);
    println!("The answer is: {}", answer);
}
