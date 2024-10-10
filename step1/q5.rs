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

    let result= a / b; 
    assert_eq!(result, 0.33333333333333333333333333333333);
    println!("{:.10}", result);
}
