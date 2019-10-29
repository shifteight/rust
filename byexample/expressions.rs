fn main() {
	let x = 5u32;

	let y = {
		let x_sq = x * x;
		let x_cu = x_sq * x;
		x_cu + x_sq + x
	};

	let z = {
		2 * x;  // `()` is assigned to `z`
	};

	println!("x is {:?}", x);
	println!("y is {:?}", y);
	println!("z is {:?}", z);
}