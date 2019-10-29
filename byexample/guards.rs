fn main() {

	let pairs = vec!{(2, -2), (1, 1), (3, 1), (4,1)};
    for pair in pairs {
    	println!("Tell me about {:?}", pair);
	    tell_about_pair(pair);
    } 

}

fn tell_about_pair(pair: (i32, i32)) {
	match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}
