
fn add_thing<T: std::ops::Add>(fst: T, snd: T) {
	let _ = fst + snd;
}

fn main() {
	add_thing(2, 2);
}