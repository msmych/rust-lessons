#[derive(Debug)]
enum List {
    BoxCons(i32, Box<List>),
    RcCons(i32, Rc<List>),
    RcRefCellCons(Rc<RefCell<i32>>, Rc<List>),
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

use crate::List::RcRefCellCons;
use crate::List::{BoxCons, Nil, RcCons};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    box_demo();
    rc_demo();
    rc_ref_cell_demo();
}

fn box_demo() {
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

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn rc_demo() {
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let rcc = RcCons(4, Rc::clone(&a));
        println!("count after creating rcc = {}", Rc::strong_count(&a));
    }
    println!(
        "count after rcc goes out of scope = {}",
        Rc::strong_count(&a)
    );
}

fn rc_ref_cell_demo() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::new(Nil)));

    let b = RcRefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcRefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
