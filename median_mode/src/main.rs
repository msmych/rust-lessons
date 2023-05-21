use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums = input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    println!("Numbers: {:?}", nums);
    println!("Median: {:?}", median(&nums));
    println!("Mode: {:?}", mode(&nums));
}

fn median(nums: &Vec<i32>) -> i32 {
    let mut v = nums.clone();
    v.sort();
    v[v.len() / 2]
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max_o = 0;
    let mut mode = 0;
    for n in nums {
        let o = map.entry(n).or_insert(0);
        *o += 1;
        if *o > max_o {
            max_o = *o;
            mode = *n;
        }
    }
    mode
}
