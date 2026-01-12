use std::thread;
use std::time::Duration;

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi from main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}
