fn main() {
    println!("Twelve Days Of Christmas");

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut i = 0;

    while i < 12 {
        let day = days[i];
        println!("");
        println!("On the {day} day of Christmas, my true love sent to me");
        if i >= 11 {
            println!("Twelve drummers drumming");
        }
        if i >= 10 {
            println!("Eleven pipers piping");
        }
        if i >= 9 {
            println!("Ten lords a-leaping");
        }
        if i >= 8 {
            println!("Nine ladies dancing");
        }
        if i >= 7 {
            println!("Eight maids a-milking");
        }
        if i >= 6 {
            println!("Seven swans a-swimming");
        }
        if i >= 5 {
            println!("Six geese a-laying");
        }
        if i >= 4 {
            println!("Five gold rings (five golden rings)");
        }
        if i >= 3 {
            println!("Four calling birds");
        }
        if i >= 2 {
            println!("Three French hens");
        }
        if i >= 1 {
            println!("Two turtledoves");
            println!("And a partridge in a pear tree");
        }
        if i == 0 {
            println!("A partridge in a pear tree");
        }
        i += 1;
    }
    println!("");
    println!("A partridge in a pear tree");
}
