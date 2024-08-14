use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    println!("\n--- Concurrency Examples ---");

    basic_threading_example();
    mutex_example();
}

fn basic_threading_example() {
    println!("\nBasic Threading Example:");
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn mutex_example() {
    println!("\nMutex Example:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}