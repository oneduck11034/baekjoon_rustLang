pub fn del(left: i16, right: i16) -> i16 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = del(3, 2);
        assert_eq!(result, 1);
    }
}
