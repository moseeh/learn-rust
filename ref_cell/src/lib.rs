pub mod messenger;

pub use messenger::Logger;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;

/// The Worker structure acts as a logger using interior mutability.
/// It stores a tracked value (an Rc<String>), a HashMap of the latest messages
/// for each log level, and a vector of all log messages.
pub struct Worker {
    /// The value whose reference count is tracked.
    pub track_value: Rc<String>,
    /// Stores the latest logged message for each type (e.g., "Info", "Warning", "Error")
    pub mapped_messages: RefCell<HashMap<String, String>>,
    /// Stores all logged messages in the order they were sent.
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    /// Creates a new Worker.
    /// The provided integer is converted to a String and wrapped in an Rc.
    pub fn new(initial: i32) -> Self {
        Worker {
            track_value: Rc::new(initial.to_string()),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

/// Implement the Logger trait for Worker. Each logging function saves the message
/// in both the `mapped_messages` and `all_messages`. Notice the use of `RefCell`
/// for interior mutability.
impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Warning: {}", msg));
    }

    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Info: {}", msg));
    }

    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Error: {}", msg));
    }
}
