fn main() {
    let v: Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }
}