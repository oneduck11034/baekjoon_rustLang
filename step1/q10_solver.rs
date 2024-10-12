pub fn multip(a: u64, b: u64) -> (u64, u64, u64, u64) {
    // took a solver book
    (a * (b % 10), a * (b % 100 / 10), a * (b / 100), a * b)
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
