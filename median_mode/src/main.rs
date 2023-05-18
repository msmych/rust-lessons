use std::collections::HashMap;
use std::io;

fn main() {
    let arr = [1, 2, 1, 4, 3, 2, 2, 3];
    println!("Array: {:?}", arr);
    println!("Median: {:?}", median(&arr));
    println!("Mode: {:?}", mode(&arr));
}

fn median(arr: &[i32]) -> i32 {
    let mut v = Vec::from(arr);
    v.sort();
    v[v.len() / 2]
}

fn mode(arr: &[i32]) -> i32 {
    let mut map = HashMap::new();
    let mut max_o = 0;
    let mut mode = 0;
    for i in arr {
        let o = map.entry(i).or_insert(0);
        *o += 1;
        if *o > max_o {
            max_o = *o;
            mode = *i;
        }
    }
    mode
}
