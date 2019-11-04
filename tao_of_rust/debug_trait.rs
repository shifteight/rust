use std::fmt::*;

struct Point {
	x: i32,
	y: i32,
}

impl Debug for Point {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn main() {
	let origin = Point {x: 0, y: 0};
	println!("The origin is: {:?}", origin);
}