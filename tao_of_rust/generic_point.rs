#[derive(Debug, PartialEq)]
struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	fn new(x: T, y: T) -> Self {
		Point {x: x, y: y}
	}
}

fn main() {
	let p1 = Point::new(1, 2);
	let p2 = Point::new("1", "2");
	println!("{:?}\n{:?}", p1, p2);
}