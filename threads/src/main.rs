use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {

	println!("Spawning threads…");
	let mut threads = vec![];
	let count = Arc::new(Mutex::new(0));

	for _i in 0..10 {
		let count = Arc::clone(&count);
		threads.push(thread::spawn(move || {
			for _j in 0..10_000 {
				let mut count = count.lock().unwrap();
				*count += 1;
			}
		}));
	}

	println!("Joining threads…");

	for thread in threads {
		thread.join().unwrap();
	}

	assert_eq!(*count.lock().unwrap(), 100_000);
}
