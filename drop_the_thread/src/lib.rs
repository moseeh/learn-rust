// Import necessary modules from the standard library.
use std::cell::{Cell, RefCell};

/// A struct to manage the states and drops of multiple threads.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,                  // Keeps track of how many threads have been dropped.
    pub states: RefCell<Vec<bool>>,          // Stores the dropped status of each thread (true if dropped).
}

impl Workers {
    /// Constructs a new `Workers` instance with default values.
    pub fn new() -> Workers {
        Workers::default()
    }

    /// Creates a new worker thread, returning its PID and the thread itself.
    pub fn new_worker(&self, c: String) -> (usize, Thread<'_>) {
        // Generate a new Thread with the current state length as PID.
        let g = Thread::new_thread(self.track_worker(), c, self);
        self.states.borrow_mut().push(false); // Mark the new thread as not dropped.
        (g.pid, g)
    }

    /// Returns the next worker ID, based on the number of current states.
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    /// Checks if a worker with the given ID has been dropped.
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    /// Marks a worker with the given ID as dropped and increments the drop counter.
    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{:?} is already dropped", id);
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

/// Represents a worker thread with a process ID, command, and reference to its parent `Workers`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,            // Process/thread ID.
    pub cmd: String,           // Command associated with this thread.
    pub parent: &'a Workers,   // Reference to the parent `Workers` struct.
}

impl<'a> Thread<'a> {
    /// Constructs a new thread given a PID, command, and parent reference.
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    /// Drops the thread explicitly by consuming `self`.
    pub fn skill(self) {
        drop(self);
    }
}

/// Automatically called when a `Thread` is dropped.
/// Updates the parent `Workers` to mark this thread as dropped.
impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}