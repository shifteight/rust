use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
	let (integer, boolean) = pair;
	(boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
	}
}

fn transpose(matrix: Matrix) -> Matrix {
	Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
	let pair = (1, true);
	println!("pair is {:?}", pair);
	println!("the reversed pair is {:?}", reverse(pair));

	println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}