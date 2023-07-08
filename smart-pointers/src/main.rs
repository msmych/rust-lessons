#[derive(Debug)]
enum List {
    BoxCons(i32, Box<List>),
    RcCons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use crate::List::{BoxCons, Nil, RcCons};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let list = BoxCons(1, Box::new(BoxCons(2, Box::new(BoxCons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let rca = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(Nil)))));
    println!("count after creating rca = {}", Rc::strong_count(&rca));
    let rcb = RcCons(3, Rc::clone(&rca));
    println!("count after creating rcb = {}", Rc::strong_count(&rca));
    {
        let rcc = RcCons(4, Rc::clone(&rca));
        println!("count after creating rcc = {}", Rc::strong_count(&rca));
    }
    println!(
        "count after rcc goes out of scope = {}",
        Rc::strong_count(&rca)
    );

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
