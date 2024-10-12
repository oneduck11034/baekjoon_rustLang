pub fn multip(a: u64, b: u64) -> (u64, u64, u64, u64) {
    let a_str= format!("{}", a);
    let b_str= format!("{}", b);

    let a_v= a_str.as_bytes();
    let b_v= b_str.as_bytes();
    
    let mut cnt= 0;
    let mut j = 1;
    for e_a in a_v.iter().rev(){
        for e_b in b_v.iter().rev(){
            let a_number: u64= (*e_a as char).to_string().parse::<u64>().unwrap();
            let b_number: u64= (*e_b as char).to_string().parse::<u64>().unwrap();
            
            // 2,5/8/3 -> 7,5/8/3 -> 4,5/8/3 
            // println!("{} // {} ", a_number, b_number);
            // 472
            // 385 
            // 10+350+2000
            
            // 10*1 + 10*2 + 10*3
            // 10*2 + 10*3 + 10*4
            // 10*3 + 10*4 + 10*5

//             10/0/0
// 16/0/0
// 6/0/0
// 0/1050/0
// 0/1680/0
// 0/630/0
// 0/0/1000
// 0/0/1600
// 0/0/600


            let mut one= 0;
            let mut two= 0;
            let mut three= 0;
            // 2360, 3776, 1416
            if j == 1{
                match cnt {
                    0 => { one= b_number*a_number; },
                    1 => { two= (10*2)*(b_number*a_number); }, 
                    2 => { three= (10*3)*(b_number*a_number); },
                    _ => {continue;}
                }
            }else if j == 2{
                match cnt {
                    0 => { one= (10*2) * b_number*a_number; },
                    1 => { two= (10*3)*(b_number*a_number); }, 
                    2 => { three= (10*4)*(b_number*a_number); },
                    _ => {continue;}
                }
            }else if j == 3{
                match cnt {
                    0 => { one= (10*3) * b_number*a_number; },
                    1 => { two= (10*4)*(b_number*a_number); }, 
                    2 => { three= (10*5)*(b_number*a_number); },
                    _ => {continue;}                   
                }
            }

            println!("{}/{}/{}", one, two, three);
        }
        
        cnt+= 1;
        if cnt >= 3 {
            cnt == 0;
        }
        j+= 1;
    }

    (0,0,0,0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multip(472,385);
        assert_eq!(result, (2360, 3776, 1416, 181720));
    }
}
