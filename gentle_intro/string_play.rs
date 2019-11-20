fn array_to_string(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let mut s = String::new();
    s.push('H');
    s.push_str("ello");
    s += ",";
    let arr = array_to_string(&[10, 20, 30]);
    let res = format!("{} {}", s, arr);
    assert_eq!(res, "Hello, [10,20,30]");
}