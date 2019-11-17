#[derive(Debug)]
struct Foo;

fn main() {
	let a = Foo;
	let closure = move || {
		let b = a;
	};
	println!("{:?}", a);
}