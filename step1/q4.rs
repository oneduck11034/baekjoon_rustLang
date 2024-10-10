pub fn multip(left: u32, right: u32) -> u32 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let result = multip(1, 2);
        assert_eq!(result, 2);
    }   

     #[test]
    fn it_works2() {
        let result = multip(3, 4);
        assert_eq!(result, 12);
    }                                               
}
