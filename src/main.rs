use std::fs;
use std::thread;

fn main() {
    let num_threads = num_cpus::get();

    let mut threads = vec![];

    for _ in 0..num_threads {
        let thread = thread::spawn(|| {
            println!("hello world");
        });

        threads.push(thread);
    }

    // Wait for all threads to finish.
    for thread in threads {
        thread.join().unwrap();
    }
}
