struct Item(u32);

impl Item {
	fn new() -> Self {
		Item(1024)
	}
	fn take_item(self) {
		// do nothing
	}
}

fn main() {
	let it = Item::new();
	it.take_item();
	println!("{}", it.0);
}