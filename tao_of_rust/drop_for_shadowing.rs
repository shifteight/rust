use std::ops::Drop;

#[derive(Debug)]
struct S(i32);
impl Drop for S {
	fn drop(&mut self) {
		println!("drop {}", self.0);
	}
}

fn main() {
	let x = S(1);
	println!("create x: {:?}", x);
	let x = S(2);
	println!("create shadowing x: {:?}", x);
}