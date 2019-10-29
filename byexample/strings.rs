fn main() {
	// A reference to a string allocated in read only memory
	let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
	println!("Pangram: {}", pangram);

	// Iterate over words in reverse, no new striing is allocated
	println!("Words in reverse");
	for word in pangram.split_whitespace().rev() {
		println!("> {}", word);
	}

	// Copy chars into a vector, sort and remove duplicates
	let mut chars: Vec<char> = pangram.chars().collect();
	chars.sort();
	chars.dedup();

	println!("chars: {:?}", chars);

	// Create an empty and growable `String`
	let mut string = String::new();
	for c in chars {
		string.push(c);
		string.push_str(", ");
	}

	// The trimmed string is a slice to the original string, hence no new
	// allocation is performed
	let chars_to_trim: &[char] = &[' ', ','];
	let trimmed_str: &str = string.trim_matches(chars_to_trim);
	println!("Uses characters: {}", trimmed_str);

	// Heap allocate a string
	let alice = String::from("I like dogs");
	// Allocate new memory and store the modified string there
	let bob: String = alice.replace("dog", "cat");

	println!("Alice says: {}", alice);
	println!("Bob says: {}", bob);
}