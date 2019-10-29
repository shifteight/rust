fn main() {
	use std::mem;

	let color = "green";

	// A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
	let print = || println!("`color`: {}", color);

	print();
	print();

	let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // let _reborrow = &mut count;  // error!

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();


    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.
    
    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}