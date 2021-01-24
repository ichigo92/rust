fn main() {

	let closure = || {
		println!("Hello, world!");
	};

	closure();

	// Passing arguments to a closure
	let closure = |num| {
		println!("{}", num);
	};

	closure(10);

	// If you want to immediately call a closure
	// Surround the closure with round brackets ()
	// and at the end add another pair of round brackets ();
	(|| {
		println!("Execute Immediately");
	})();

}
