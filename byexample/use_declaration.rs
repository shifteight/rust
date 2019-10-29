use deeply::nested::function as other_func;

fn function() {
	println!("called `function()`");
}

mod deeply {
	pub mod nested {
		pub fn function() {
			println!("called `deeply::nested::function()`");
		}
	}
}

fn main() {
	// Easier access to `deeply::nested::function`
	other_func();

	println!("Entering block");
	{
		// this is equivalent to `use deeply::nested::function as function`.
		// this `function()` will shadow the outer one.
		use deeply::nested::function;
		function();

		// `use` bindings have a local scope. In this case, the 
		// shadowing of `function()` is only in this block.
		println!("Leaving block");
	}

	function();
}