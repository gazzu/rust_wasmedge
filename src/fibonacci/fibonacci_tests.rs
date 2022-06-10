#[cfg(test)]
mod fibonacci_tests {
    use super::super::fibonacci;

    #[test]
    fn fibonacci_test() {
      match fibonacci::fib(8) {
        Ok(number)  => assert_eq!(number, 21),
        Err(_) => return,
      };
    }
}