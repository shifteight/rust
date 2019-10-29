fn main() {
    let mut s = String::from("hello");

    let mut len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    change(&mut s);
    len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}