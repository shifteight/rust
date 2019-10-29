// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F)  where F: FnOnce() {
	f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
	f(3)
}
fn main() {
	use std::mem;

	let greeting = "hello";
	let mut farewell = "goodbye".to_owned();
	let diary = || {
		// `greeting` is by reference: requires `Fn`.
		println!("I said {}.", greeting);

		// Mutation forces `farewell` to be captured by
		// mutable reference. Now requires `FnMut`.
		farewell.push_str("!!!");
		println!("Then I screamed {}.", farewell);
		println!("Now I can sleep. zzzzz");

		// Manually calling drop forces `farewell` to
		// be captured by value. Now requires `FnOnce`.
		mem::drop(farewell);
	};

	// Call the function which applies the closure
	apply(diary);

	// `double` satisfies `apply_to_3`'s trait bound
	let double = |x| 2 * x;

	println!("3 doubled: {}", apply_to_3(double));
}