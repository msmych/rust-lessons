fn main() {
    background_color();
    number_stack();
    print_coordinates(&(3, 5));
    multiple_patterns();
    matching_ranges();
    desctructuring();
    match_guard();
    at_bindings();
}

fn background_color() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using orange as the background color");
    }
}

fn number_stack() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn desctructuring() {
    let p = Point { x: 0, y: 7, z: -5 };

    let Point { x: a, y: b, .. } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y, .. } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0, .. } => println!("On the x axis at {x}"),
        Point { x: 0, y, .. } => println!("On the y axis at {y}"),
        Point { x, y, .. } => println!("On neither axis: ({x}, {y})"),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    let ((feet, inches), Point { x, y, z }) = ((3, 10), Point { x: 3, y: -10, z: 4 });

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
        (first, .., last) => println!("Some numbers: {first}, {last}"),
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings() {
    let msg = Message::Move { x: 1, y: 2 };

    match msg {
        Message::Move {
            x: x_var @ 0..=10, ..
        } => println!("Moving to x in range: {}", x_var),
        Message::Move { x: 20..=40, .. } => println!("Moving to x another range"),
        Message::Move { x, .. } => println!("Moving to some other x: {}", x),
        _ => println!("Not moving"),
    }
}
