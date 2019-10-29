use std::mem;

fn analyze_slice(slice: &[i32]) {
	println!("first element of the slice: {}", slice[0]);
	println!("the slice has {} elements", slice.len());
}

fn main() {
	let xs = [1, 2, 3, 4, 5];
	let ys = [0; 500];

	// Arrays are stack allocated
	println!("array occupies {} bytes", mem::size_of_val(&ys));

	// Arrays can be automatically borrowed as slices
	println!("borrow the whole array as a slice");
	analyze_slice(&xs);

	// Slices can point to a section of an array
	println!("borrow a section of the array as a slice");
	analyze_slice(&ys[1..4]);

	// out of bound indexing causes compile error
	// println!("{}", xs[5]);
}