// 解引用会获得所有权

fn join(s: &String) -> String {
	let append = *s;
	"Hello".to_string() + &append
}
fn main() {
	let x = " world".to_string();
	join(&x);
}

// error[E0507]: 
// cannot move out of `*s` which is behind a shared reference