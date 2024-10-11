use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let input= buff.next().unwrap().unwrap();

    let result= format!("{}{}", input, "??!");

    assert_eq!(result, "joonas??!".to_string());
    println!("{}", result); 
}
