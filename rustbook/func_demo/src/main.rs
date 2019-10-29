fn main() {

    let f = add_one;

    println!("f(1) = add_one(1) = {}", f(1));
    // diverges();

}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

// fn diverges() -> ! {
//     panic!("This function never returns!");
// }
