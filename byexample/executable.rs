// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
	rary::public_function();

	// rary::private_function();
	rary::indirect_access();
}

// Compile: rustc executable.rs --extern rary=library.rlib