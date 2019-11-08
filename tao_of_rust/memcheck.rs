fn create_box() {
	let box3 = Box::new(3);
}

fn main() {
	let box1 = Box::new(1);
	{
		let box2 = Box::new(2);
	}
	for _ in 0..1_000 {
		create_box();
	}
}