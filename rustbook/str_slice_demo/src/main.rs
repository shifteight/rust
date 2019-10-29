fn main() {
    // using String slice
    let s1 = String::from("hello world");
    let word = first_word(&s1[..]);
    println!("the first word is: {}", word);

    // using string literal slice
    let s2 = "hello world";
    let word = first_word(&s2[..]);
    println!("the first word is: {}", word);

    // using string literal instead of slice
    let word = first_word(s2);
    println!("the first word is: {}", word);
    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}