use std::sync::{Condvar, Mutex, PoisonError};

/// A counting limit on requests in flight, shared by every thread.
pub struct Gate {
    used: Mutex<usize>,
    free: Condvar,
    max: usize,
}

/// One slot of a [`Gate`], given back when dropped.
pub struct Pass<'a>(&'a Gate);

impl Gate {
    pub const fn new(max: usize) -> Gate {
        Gate {
            used: Mutex::new(0),
            free: Condvar::new(),
            max,
        }
    }

    /// Blocks until a slot is free. A poisoned lock only means another
    /// download panicked; the count itself is still right.
    pub fn enter(&self) -> Pass<'_> {
        let mut used = self.used.lock().unwrap_or_else(PoisonError::into_inner);
        while *used >= self.max.max(1) {
            used = self.free.wait(used).unwrap_or_else(PoisonError::into_inner);
        }
        *used += 1;
        Pass(self)
    }
}

impl Drop for Pass<'_> {
    fn drop(&mut self) {
        let mut used = self.0.used.lock().unwrap_or_else(PoisonError::into_inner);
        *used = used.saturating_sub(1);
        self.0.free.notify_one();
    }
}
