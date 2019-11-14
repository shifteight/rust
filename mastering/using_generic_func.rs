use std::str;
fn main() {
    let num_from_str = str::parse::<u8>("32").unwrap();
    println!("Parsed number {}", num_from_str);
}