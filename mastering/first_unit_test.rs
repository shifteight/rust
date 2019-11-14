#[test]
fn basic_test() {
	assert!(true);
}

// rustc --test first_unit_test.rs
// If we want to run the test in single thread mode, run:
// RUST_TEST_THREAD=1 ./first_unit_test