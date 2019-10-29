#[derive(Debug)]
struct Person<'a> {
	name: &'a str,
	age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
	x: f32,
	y: f32,
}

// structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
	p1: Point,
	p2: Point,
}

fn main() {
	// create struct with field init shorthand
	let name = "Peter";
	let age = 27;
	let peter = Person { name, age };

	println!("{:?}", peter);

	let point: Point = Point { x: 0.3, y: 0.4 };
	println!("point coordinates: ({}, {})", point.x, point.y);

	let new_point = Point { x: 0.1, ..point };
	println!("second point: ({}, {})", new_point.x, new_point.y);

	// destructure the point using a `let` binding
	let Point { x: my_x, y: my_y } = point;

	let _rectangle = Rectangle {
		p1: Point { x: my_y, y: my_x },
		p2: point,
	};

	let _nil = Nil;

	let pair = Pair(1, 0.1);

	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	let Pair(integer, decimal) = pair;

	println!("pair contains {:?} and {:?}", integer, decimal);

	let p1: Point = Point { x: 3.0, y: 4.0 };
	let p2: Point = Point { x: 6.0, y: 10.0 };
	let rect1: Rectangle = Rectangle { p1: p1, p2: p2 };
	println!("area of {:?}: {}", &rect1, rect_area(&rect1));


	println!("{:?}", rect1);

	let side = 3.0;
	let p = Point { x: 1.0, y: 1.0 };
	let square = square(p, side);
	println!("{:?} is a square", square);

}

fn rect_area(rect: &Rectangle) -> f32 {
	let Rectangle { p1: Point { x: x1, y: y1 },
	                p2: Point { x: x2, y: y2 } }
	    = rect;
	(y2 - y1) * (x2 - x1)
}

fn square(p: Point, side: f32) -> Rectangle {
	let p2 = Point { x: p.x + side, y: p.y + side };
	Rectangle { p1: p, p2: p2 }
}