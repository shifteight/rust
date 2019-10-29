mod my {
	// A public struct with a public field of generic type `T`
	pub struct OpenBox<T> {
		pub contents: T,
	}

	// A public struct with a private field of generic type `T`
	#[allow(dead_code)]
	pub struct ClosedBox<T> {
		contents: T,
	}

	impl<T> ClosedBox<T> {
		// A public constructor method
		pub fn new(contents: T) -> ClosedBox<T> {
			ClosedBox {
				contents: contents,
			}
		}
	}
}

fn main() {
	// Public structs with public fields can be constructed as usual
	let open_box = my::OpenBox { contents: "public information" };

	// and their fields can be normally accessed.
	println!("The open box contains: {}", open_box.contents);

	let _closed_box = my::ClosedBox::new("classified information");

	// println!("The closed box contains: {}", _closed_box.contents);  // error
}