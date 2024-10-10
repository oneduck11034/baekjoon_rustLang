pub fn solution() -> String {
    let hello= "Hello World!";

    hello.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution();
        assert_eq!(result, "Hello World!".to_string());
    }
}
