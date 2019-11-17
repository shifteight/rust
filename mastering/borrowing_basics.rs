#[derive(Debug)]
struct Foo(u32);

fn main() {
	let foo = Foo(1024);
	let bar = &foo;
	println!("Foo is {:?}", foo);
	println!("Bar is {:?}", bar);
}