fn main() {
    greet();
}

fn greet() -> String {
    "Hello, world!".to_string()
}

#[test]
fn test_greet() {
    assert_eq!("Hello, world!", greet())
}