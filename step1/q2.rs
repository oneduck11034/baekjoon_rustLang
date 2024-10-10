// 첫째 줄에 A와 B가 주어진다. (0 < A, B < 10)

pub fn add(left: i16, right: i16) -> i16 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(1, 2);
        assert_eq!(result, 3_i16);
    }
}
