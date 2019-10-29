pub fn hello() -> String {
    ("Hello, world!").to_string()
}

#[cfg(test)] // only compiles when runing tests
mod tests { // seperates tests from code
  use super::hello; // import root hello function

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}