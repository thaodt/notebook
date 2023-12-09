use chap31::Semaphore;
use std::sync::Arc;
use std::thread;

fn main() {
    // Create the semaphore with no allowance initially.
    // `Arc` would not be necessary if the thread is scoped, but std threads must have 'static lifetime.
    let semaphore = Arc::new(Semaphore::new(0));

    {
        let s = Arc::clone(&semaphore);
        thread::spawn(move || {
            println!("child");
            // allows the parent thread to proceed
            s.post().unwrap();
        });
    }

    if let Err(err_msg) = semaphore.wait() {
        eprintln!("{}", err_msg);
    }
    // parent will always print after child
    println!("parent");
}
