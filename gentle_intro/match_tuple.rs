fn match_tuple(t: (i32, String)) {
    let text = match t {
        (0, s) => format!("zero {}", s),
        (1, ref s) if s == "hello" => format!("hello one!"),
        tt => format!("no match {:?}", tt),
    };
    println!("{}", text);
}

fn main() {
    match_tuple((1, "hello".to_string()));
    match_tuple((0, "xxx".to_string()));
    match_tuple((2, "".to_string()));
}
