use std::thread;
use std::time::Duration;

fn main() {
	// handle variable is what is known as JoinHandle
	// the return type of thread::spawn is JoinHandle
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});

	// the main thread will wait for the spawned thread
	// to finish and then run its for loop, so the output
	// won't be interleaved anymore
	handle.join().unwrap();

	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}

	// this will make main thread wait for
	// the spawn thread to finish executing
	// handle.join().unwrap();
}