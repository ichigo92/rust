use std::thread;


fn main() {

	let v = vec![1, 2, 3];

	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", v);
	});

	// Will throw an error, compiler can’t tell
	// how long the spawned thread will run, so
	// it doesn’t know if the reference to v will
	// always be valid
	/*let handle = thread::spawn(|| {
		println!("Here's a vector: {:?}", v);
	});*/

	// If you uncomment below code, with move in
	// spawn thread closure, you will get an error
	// as the variable v has moved into the closure
	// we can no longer call drop on it in the main thread
	// drop(v);	// main thread drops v, v is no longer valid

	handle.join().unwrap();
}