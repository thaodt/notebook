//! Counting semaphore to control concurrent access to a fixed number of shared resource.
//! Figure 31.17: Implementing Semaphores With Locks And CVs of OSTEP.
//!
//! # Examples
//! ```
//! use semaphore::Semaphore;
//! use std::sync::Arc;
//! use std::thread;
//!
//! // Create the semaphore with no allowance initially.
//! // `Arc` would not be necessary if the thread is scoped, but std threads must have 'static lifetime.
//! let semaphore = Arc::new(Semaphore::new(0));
//!
//! {
//!     let s = Arc::clone(&semaphore);
//!     thread::spawn(move || {
//!         println!("child");
//!         // allows the parent thread to proceed
//!         s.post().unwrap();
//!     });
//! }
//!
//! semaphore.wait();
//! // parent will always print after child
//! println!("parent");
//! ```
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

pub struct Semaphore {
    cond: Condvar,
    limit: Mutex<u32>,
}

impl Semaphore {
    /// Create a new semaphore which limits the maximum number of threads having access to a shared
    /// resource, usually entrance to the critical section or access to shared memory.
    pub fn new(limit: u32) -> Self {
        Semaphore {
            cond: Condvar::new(),
            limit: Mutex::new(limit),
        }
    }

    /// Wait on the semaphore until the resources are available. Block the calling thread when the
    /// number of threads accessing the resources is equal ot or more than allowed.
    pub fn wait(&self) -> Result<(), PoisonError<MutexGuard<u32>>> {
        let mut value = self.limit.lock()?;
        while *value == 0 {
            value = self.cond.wait(value)?;
        }

        *value -= 1;
        Ok(())
    }

    /// Release the semaphore, allowing other threads to have access to the resources.
    pub fn post(&self) -> Result<(), PoisonError<MutexGuard<u32>>> {
        let mut value = self.limit.lock()?;
        *value += 1;
        self.cond.notify_one();
        Ok(())
    }
}
