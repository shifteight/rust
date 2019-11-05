// fn main() {
// 	let x: Result<i32, &str> = Ok(-3);
// 	assert_eq!(x.is_ok(), true);
// 	let x: Result<i32, &str> = Err("Some error");
// 	assert_eq!(x.is_ok(), false);
// }

use std::fs::File;
fn main() -> Result<(), std::io::Error> {
	let f = File::open("bar.txt")?;
	Ok(())
}