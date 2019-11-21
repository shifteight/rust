fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64) -> f64 {
    f(x)
}

fn main() {
    let m = 2.0;
    let c = 1.0;
    
    let lin = |x| m*x + c;
    println!("res {} {}", lin(1.0), lin(2.0));
    
    let res1 = apply(3.0, lin);
    assert_eq!(res1, 7 as f64);
    let l = lin;
    assert_eq!(l(4.0), 9 as f64);
    let res2 = apply(3.14, |x| x.sin());
    println!("res2 {:.2}", res2);
    
}