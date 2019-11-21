fn mutate<F>(mut f: F)
where
    F: FnMut(),
{
    f()
}

fn main() {
    let mut s = "world";
    mutate(|| s = "hello");
    assert_eq!(s, "hello");

    let mut changer = || s = "world";
    changer();
    assert_eq!(s, "world");
}
