pub fn multip(a: u64, b: u64) -> (u64, u64, u64, u64) {
    let a_str= format!("{}", a);
    let b_str= format!("{}", b);

    let a_v= a_str.as_bytes();
    let b_v= b_str.as_bytes();
    
    let mut cnt= 0;
    for e_a in a_v.iter().rev(){
        for e_b in b_v.iter().rev(){
            let a_number: u64= (*e_a as char).to_string().parse::<u64>().unwrap();
            let b_number: u64= (*e_b as char).to_string().parse::<u64>().unwrap();
            
            // 10*1 + 10*2 + 10*3
            // 10*4 + 10*5 + 10*6

        }

        if cnt >= 3 {
            cnt == 0;
        }
        cnt+= 1;
    }

    (0,0,0,0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multip(2, 2);
        assert_eq!(result, (2360, 3776, 1416, 181720));
    }
}
