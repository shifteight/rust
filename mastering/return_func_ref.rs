fn get_a_borrowed_value() -> &'static u8 {
	let x = 1;
	&x
}

fn main() {
	let value = get_a_borrowed_value();
}