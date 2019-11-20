fn main() {
	let ints = [1, 2, 3, 4, 5];
	let slice = &ints;
	
	for w in slice.windows(2) {
		println!("window {:?}", w);
	}
	
	for c in slice.chunks(2) {
		println!("chunk {:?}", c);
	}
}
