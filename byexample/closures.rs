fn main() {
	fn function             (i: i32) -> i32 { i + 1 }

	let closure_annotated = |i: i32| -> i32 { i + 1 };
	let closure_inferred  = |i     |          i + 1  ;

	let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    
    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred
    let one = || 1;
    println!("closure returning one: {}", one());
}