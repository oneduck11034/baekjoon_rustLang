use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buff= std.lock().lines();
    let input= buff.next().unwrap().unwrap();
    
    let buddhist_calendar= input.trim().parse().unwrap_or(0);
    let america= buddhist_calendar - 543;

    assert_eq!(america, 1998_u32);
    println!("{}", america); 
}
