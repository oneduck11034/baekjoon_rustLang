use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let a_and_b: Vec<i32>= buff.next().unwrap().unwrap()
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();
    
    let a= a_and_b[0] as f32;
    let b= a_and_b[1] as f32;

    println!("{}", a+b);
    println!("{}", a-b);
    println!("{}", a*b);
    println!("{:.10}", a/b);
    println!("{}", a%b);
}
