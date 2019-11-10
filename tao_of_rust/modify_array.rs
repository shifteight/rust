fn foo(v: &mut [i32; 3]) {
	v[0] = 3;
}
fn main() {
	let mut v = [1, 2, 3];
	foo(&mut v);
	assert_eq!([3, 2, 3], v);
}